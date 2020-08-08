# Bartender

This project is a bartender machine.

The idea behind is that you will be able to select a drink from several different recipes and then it will mix it togheter and pour it into a glass.

This repo contains all related code that will run on the actual machine.

It contains an app for selecting recipes, configuring pumps and starting the pouring process.
It will also contain the backend to control all pumps and such.

This project is also depended on my other project Recipe Maker.
I wanted to keep Recipe Maker as a seperate project due to Recipe Maker is more a general project which I have other plans for.
So this project will only contain code/information regarding the actual machine. Management of recipes will be done through Recipe Maker.

## TODO

 - [ ] Clean up code, remove old components, proper naming, there is probably some bad code too
 - [ ] Documentation of both the code and the machine/process
 - [x] Find a better name, probably better to seperate this from recipe-maker project.
 - [ ] Add details to hardware
 - [ ] Hide/Show only recipes that the machine can produce (based on ingredients from pumps)

## Hardware

_Needs updating_

-   LCD screen with touch functionality
-   Raspberry Pi/Alternatives
-   10x Pumps
-   Case

## Roadmap

The roadmap below will contain a list of features that I have planned for different version.
It will also list features that I want to do in the future but isn't planned for specific versions.

As version 1 isn't completed, some of the features might be moved between version 1 and 2.

Version 1 will be more of a prototype, it will contain the basics to mix a drink.
The basics will be that it will contain a few pumps that can pump liquid from A to B. Meaning it can pump liquid from a bottle to a glass.
So there wont be any sort of "mixing" or "shaking" and such.
The idea is to keep it simple from the start and then add features when needed.

For version 1 there will also be a simple GUI to select drinks and do some simple configuration.

Version 2 will most likely be a bunch of bug fixes and possibly a new feature or two.
It probably wont be a huge update in terms of the actual machine, but more updates related to coding.

After Version 2 is completed and I feel happy with the base, I will add new features such as shaking, mixing, cooling, etc.

### Version 1

-   GUI
    -   Recipes is filtered based on what liquid we can pump
    -   Select a recipe
    -   Select the volume of your glass/drink
    -   Start pouring
    -   Change what liquid/ingredient is in each pump
    -   Caching of recipes. Mainly it should use graphql but in case of no internet or mobile data there should be cache.

-   Machine
    -   Pump liquid from A to B

### Version 2

-   GUI

    -   Custom volume selection
    -   More settings
        -   Control how many pumps we have
        -   Control what pump is what GPIO

-   Machine

### Future

-   GUI
    -   Filter/Search pumps
    -   Stats
        - Usage per recipe/ingredient/total
        - Volume poured per recipe/ingredient/total
        - Total bandwidth - Cache/Non cache
        - Viewed per recipe/ingredient/total
        - How many times each pump has changed ingredient
        - Pump usage volume and times

-   Machine
    -   RGB Everywhere
    -   Process a recipe step by step (pour different liquid at different times)
    -   Shake the drink
    -   Stir the drink
    -   Ice/Cooler for liquid
    -   Lime/Lemon, add a holder for lime/lemon. Maybe in the future it can cut them too.
    -   Sugar/Salt around the glass
    -   Make it possible to add spices
    -   Auto detection/prevention
        -   Detect if there is an actual glass to pour into or not
        -   Detect if a pump has an actual liquid
        -   Detect glass size, so we don't pour too much by accident
        -   Detect volume of the liquid, so we can show a percentage for each pump and warn when it's low
        -   Detect different liquid, scan bottles with QR or similar. Maybe RFID tag

 - Updater
