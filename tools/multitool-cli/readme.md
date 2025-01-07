# Multitool CLI
#### all in one cli for all your angry birds epic modding needs

## Features
- Balancing data, Locale and Player data decoding/encoding
- Supports multiple output formats (Ron, Json, Csv (locale only))
- Kinda documented
- Automatically extracts and reinserts data in to player prefs xml file
- uhhh its written in rust?
- can probably run on linux/macos? (not tested)
- does not require any external runtime

## Usage

Cli is split into 3 parts
- balancing - both event and regular balancing data containers
- prefs - player_prefs.xml
- locale - localization files (loca)

depending on what you need you have to use the correct subcommand
```
abe_multitool.exe help

-------------------------------------------------------------------------------

Cli based multi-tool for a 2014 game developed by Rovio called Angry Birds Epic

Usage: abe_multitool.exe <COMMAND>

Commands:
  balancing  Encode or decode a serialized balancing data container
  prefs      Encode or decode a xml player prefs file
  locale     Encode or decode a language locale file
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

### Examples
#### Decoding/encoding player data with json
```
abe_multitool.exe prefs .\com.rovio.gold.v2.playerprefs.xml .\decoded_player_data.json decode
----
abe_multitool.exe prefs .\com.rovio.gold.v2.playerprefs.xml .\decoded_player_data.json encode .\encoded_player_prefs.xml
```

#### Decoding/encoding player data with ron
```
abe_multitool.exe prefs .\com.rovio.gold.v2.playerprefs.xml .\decoded_player_data.ron decode -O=ron
----
abe_multitool.exe prefs .\com.rovio.gold.v2.playerprefs.xml .\decoded_player_data.ron encode .\encoded_player_prefs.xml
```

#### Decoding/encoding balancing data container with json
```
abe_multitool.exe balancing .\live_SerializedBalancingDataContainer_3.0.1.bytes SkillBalancingData decode
----
abe_multitool.exe balancing .\live_SerializedBalancingDataContainer_3.0.1.bytes SkillBalancingData encode .\ABH.Shared.BalancingData.SkillBalancingData.json .\encoded_balancing.bytes
```

#### Decoding/encoding balancing data container with ron
```
abe_multitool.exe balancing .\live_SerializedBalancingDataContainer_3.0.1.bytes SkillBalancingData decode -O=ron
----
abe_multitool.exe balancing .\live_SerializedBalancingDataContainer_3.0.1.bytes SkillBalancingData encode .\ABH.Shared.BalancingData.SkillBalancingData.ron .\encoded_balancing.bytes
```

#### Decoding every balancing data container with json
```
abe_multitool.exe balancing .\live_SerializedBalancingDataContainer_3.0.1.bytes decode --all
```

#### Decoding every balancing data container with ron
```
abe_multitool.exe balancing .\live_SerializedBalancingDataContainer_3.0.1.bytes decode --all -O=ron
```

#### Decoding/encoding localization data with json
```
abe_multitool.exe locale decode .\live_English.bytes .\decoded_locale.json
----
abe_multitool.exe locale encode .\decoded_locale.json .\encoded_english.bytes
```

#### Decoding/encoding localization data with ron
```
abe_multitool.exe locale decode .\live_English.bytes .\decoded_locale.ron -O=ron
----
abe_multitool.exe locale encode .\decoded_locale.ron .\encoded_english.bytes
```

#### Decoding/encoding localization data with csv
```
abe_multitool.exe locale decode .\live_English.bytes .\decoded_locale.csv -O=csv
----
abe_multitool.exe locale encode .\decoded_locale.csv .\encoded_english.bytes
```

## Differences compared to IGTBAP Tools
- no interactive prompts, just pure cli
- requires using an xml file for player data
- all names are `camelCase` instead of `PascalCase`
- Enums are strings instead of numbers, and they are named differently (more C like) <br>
  Container: `BuyableShopOfferBalancingData`
  - igtbap's "ABEpicBalancingDataContainerDecoder"
    ```json
    {
      "RequirementType": 4,
      "NameId": "bird_red",
      "Value": 1.0
    }
    ```
  - this cli
    ```json
    {
      "requirementType": "HAVE_BIRD",
      "nameId": "bird_red",
      "value": 1.0
    }
    ```
- Some maps are key value pairs instead (only affects player data) <br>
  - igtbap's "ABEpicPlayerDecoder"
    ```json
    {
      "LocationProgress": {
        "World": 0,
        "ChronicleCave": 0
      }
    }
    ```
    this cli
    ```json
    {
      "locationProgress": [
        {
          "key": "LOCATION_TYPE_WORLD",
          "value": 0
        },
        {
          "key": "LOCATION_TYPE_CHRONICLE_CAVE",
          "value": 0
        }
      ]
    }
    ``` 

## Known Issues
- Inf and NaN values are getting nulled out in json output, use ron output as a alternative with `-O=ron` flag (affects only player data from my testing)

## Credits
- igtbap - Providing all proto definitions