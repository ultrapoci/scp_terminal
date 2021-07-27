SCP Terminal
===

SCP Terminal is a simple terminal application written in Rust that lets user to browse the [SCP Foundation wiki](https://scp-wiki.wikidot.com) from terminal. 

## Installation and Usage

If you have the Rust compiler installed, simply clone this repository and run
```
cargo build --release
```
inside the downloaded directory to compile the program. After that, you can run
```
cargo run --release -- <scp-code>
```
to run the application and browse the wiki page of the SCP indicated by \<scp-code\>. For example:
```
cargo run --release 035
```
will show [SCP-035](https://scp-wiki.wikidot.com/scp-035)'s page. Note that at the moment the application is sensible to trailing zeros, meaning that "35" in the example above won't work. 

## Suggestions for best experience

Here's how you can make reading the wiki through terminal more appealing. 
1. If you're on Windows 10, download the new [Windows Terminal](https://www.microsoft.com/it-it/p/windows-terminal/9n0dx20hk701?rtc=1&activetab=pivot:overviewtab). This is simply a better terminal than the default one and is safely available on Microsoft Store.
2. Open the new Windows Terminal, click the downward arrow above (near the tabs) and click on settings.
3. Here you can modify the behaviour of various shells. Add a new profile by  cloning Windows PowerShell. 
4. Open the settings for the newly created shell by clicking "Windows PowerShell (cloned)" on the left. 
5. On the right you will see various settings. Click on the second tab (it should be called "Appeareance") and switch on "Retro terminal effects". You can also increase the font size a bit to make it more readable.
   
Now you can use SCP Terminal from this Windows PowerShell modified to look like an old terminal from the 70's!.
