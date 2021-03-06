# Huddle

A small ncurses terminal HUD.

<p align="center">
  <img src="https://raw.githubusercontent.com/millerjs/huddle-rs/master/resources/screen1.png" alt="Current appearance"/>
</p>

## Installation

```bash
$ git clone https://github.com/millerjs/huddle.git
$ cd huddle
$ cargo build
```

## Usage

```bash
$ cargo run config.json
```

Example `config.json` file:
```JSON
{
    "delay": 1000,
    "root": [
        {
            "type": "BigText",
            "color": "WHITE_ON_BLACK",
            "x_offset": 0,
            "y_offset": 1,
            "height": 7,
            "width": 50,
            "border-vertical": 0,
            "border-horizontal": 0,
            "content": {
                "type": "Subcommand",
                "source": "date +%l:%M"
            }
        },
        {
            "type": "BigText",
            "color": "GREEN_ON_BLACK",
            "x_offset": 52,
            "y_offset": 1,
            "height": 7,
            "width": 35,
            "border-vertical": 0,
            "border-horizontal": 0,
            "content": {
                "type": "Subcommand",
                "source": "./get_battery.sh"
            }
        }
    ]
}
```

## Contributing

Really? Okay!  Send a PR my way.

## License

The MIT License (MIT)
