#[macro_export]
macro_rules! generate_container_encode_match {
    ($archive:expr, $key:expr, $data:expr, $dataformat:expr, $($variant:ident => $proto_type:ty),+ $(,)?) => {
        match $key {
            $(
                BalancingDataTypes::$variant => {
                    let data: $proto_type = match $dataformat {
                        DataFormat::Json => serde_json::from_str($data)?,
                        DataFormat::Ron => {
                            //todo: there is def a less stupid way of doing this
                            let options = ron::Options::default().with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME);
                            options.from_str($data)?
                        }
                    };
                    $archive.set_data_enum::<$proto_type>($key, data)?;
                }
            )+
        }
    };
}

#[macro_export]
macro_rules! generate_key_to_json_match {
    ($reader:expr, $key:expr, $($variant:ident => $proto_type:ty),+ $(,)?) => {
        match $key {
            $(
                BalancingDataTypes::$variant => {
                    let data = $reader.get_data_enum_decoded::<$proto_type>($key)?;
                    serde_json::to_string_pretty(&data)?
                },
            )+
        }
    };
}

#[macro_export]
macro_rules! generate_key_to_ron_match {
    ($reader:expr, $key:expr, $($variant:ident => $proto_type:ty),+ $(,)?) => {
        match $key {
            $(
                BalancingDataTypes::$variant => {
                    let data = $reader.get_data_enum_decoded::<$proto_type>($key)?;
                    ron::ser::to_string_pretty(&data, Default::default())?
                },
            )+
        }
    };
}

#[macro_export]
macro_rules! generate_reencode_container_data_unsafe {
    ($old_archive:expr, $new_archive:expr, $key:expr, $($variant:ident => $proto_type:ty),+ $(,)?) => {
        match $key {
            $(
                BalancingDataTypes::$variant => {
                    let data: $proto_type = $old_archive.get_data_enum_decoded::<$proto_type>($key).unwrap();
                    $new_archive.set_data_enum::<$proto_type>($key, data).unwrap();
                }
            )+
        }
    };
}
