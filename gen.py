#!/usr/bin/env python

# generate Rust code for themes from YAML files

import yaml


def hex_to_rgb_tuple(hex_string: str):
    return tuple(int(hex_string[i + 1 : i + 3], 16) for i in (0, 2, 4))


if __name__ == "__main__":
    with open("orig.yml", "r") as stream:
        parsed = yaml.safe_load(stream)

        print("Theme {")

        print(
            "background: RGB8::new{},".format(
                hex_to_rgb_tuple(parsed["colors"]["primary"]["background"])
            )
        )

        print(
            "foreground: RGB8::new{},".format(
                hex_to_rgb_tuple(parsed["colors"]["primary"]["foreground"])
            )
        )

        # BEGIN NORMAL

        print("normal: ThemeColors {")
        print(
            "black: RGB8::new{},".format(
                hex_to_rgb_tuple(parsed["colors"]["normal"]["black"])
            )
        )
        print(
            "red: RGB8::new{},".format(
                hex_to_rgb_tuple(parsed["colors"]["normal"]["red"])
            )
        )
        print(
            "green: RGB8::new{},".format(
                hex_to_rgb_tuple(parsed["colors"]["normal"]["green"])
            )
        )
        print(
            "yellow: RGB8::new{},".format(
                hex_to_rgb_tuple(parsed["colors"]["normal"]["yellow"])
            )
        )
        print(
            "blue: RGB8::new{},".format(
                hex_to_rgb_tuple(parsed["colors"]["normal"]["blue"])
            )
        )
        print(
            "magenta: RGB8::new{},".format(
                hex_to_rgb_tuple(parsed["colors"]["normal"]["magenta"])
            )
        )
        print(
            "cyan: RGB8::new{},".format(
                hex_to_rgb_tuple(parsed["colors"]["normal"]["cyan"])
            )
        )
        print(
            "white: RGB8::new{},".format(
                hex_to_rgb_tuple(parsed["colors"]["normal"]["white"])
            )
        )

        print("},")
        # END NORMAL

        # BEGIN BRIGHT
        print("bright: ThemeColors {")
        print(
            "black: RGB8::new{},".format(
                hex_to_rgb_tuple(parsed["colors"]["bright"]["black"])
        )
        
        print(
            "red: RGB8::new{},".format(
                hex_to_rgb_tuple(parsed["colors"]["bright"]["red"])
            )
        )
        print(
            "green: RGB8::new{},".format(
                hex_to_rgb_tuple(parsed["colors"]["bright"]["green"])
            )
        )
        print(
            "yellow: RGB8::new{},".format(
                hex_to_rgb_tuple(parsed["colors"]["bright"]["yellow"])
            )
        )
        print(
            "blue: RGB8::new{},".format(
                hex_to_rgb_tuple(parsed["colors"]["bright"]["blue"])
            )
        )
        print(
            "magenta: RGB8::new{},".format(
                hex_to_rgb_tuple(parsed["colors"]["bright"]["magenta"])
            )
        )
        print(
            "cyan: RGB8::new{},".format(
                hex_to_rgb_tuple(parsed["colors"]["bright"]["cyan"])
            )
        )
        print(
            "white: RGB8::new{},".format(
                hex_to_rgb_tuple(parsed["colors"]["bright"]["white"])
            )
        )

        print("},")
        # END BRIGHT

        print("},")

