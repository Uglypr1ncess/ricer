## MACICE


a simple rust program that changes other program values so you can have matching themes in macos


supported programs:

neovim
alacritty


how does it do it?


macice first checks the root cv, if the needed values are filled it will begin looking through each program and its filepath to change   the values to a selected theme. If the json is not filled macice will try finding the paths by itself, when it doesnt find the paths it    will ask the user to enter the config paths and then store them to the cv



step by step


create base file structure:

src/
--/main.rs
--/json_handler.rs

