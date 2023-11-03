# Hello Guardian 

Welcome to the Oracle Engine wiki! This page will cover how to get any necessary prequisites, run the given sample script, and show how to change the weapon used in the script.

## Prerequisites 

* Rust v1.65 
* Node v18  
* WebAssembly (wasm-pack)
* Bungie API key

## Installing / Getting Prequisites

On macOS and Linux you can use leverage [homebrew](https://brew.sh/) to install rust, node, and wasm-pack via brew install < name of app >

You can get your own Bungie API key by registering an application on [Bungie's developer portal](https://www.bungie.net/en/Application)

## Building Oracle Engine for NodeJS

In order to test the changes you may want to make to Oracle Engine, you will have to build it for NodeJS. 

```wasm-pack build --release --target nodejs```

We have included a sample script in this wiki that you can use as a reference. Please remove the .sample file extension prior to use. In order to use the script, you will need to supply your own Bungie API key. Once you have done that, you can run the script via node. Make sure that the path for D2Calc points to wherever your oracle package is! With that in mind, running the following command should output Enigma's Draw as Oracle Engine defines it.

```node docs/build.js```

The reason why Enigma's Draw is shown is because its hash (1723380073) is what is used in the script. Go ahead and change this to whatever weapon you wish! Heres a few of my fav weapons in Destiny 2 to get you started.

* 3871226707 // Mechabre
* 578105049 // Oversoul Edict (Adept)
* 235827225 // Eyasluna

## Tweaking the Script to Test Things

I know it's hard to believe that Enigma's Draw isn't the only weapon in Destiny 2 and even if it was... what about testing various perks on it? In order to do that, we need to leverage some of the functions inside of src/lib.rs which will be covered in a different wiki page.