SCP Terminal
===

SCP Terminal is a simple terminal application written in Rust that lets user to browse the [SCP Foundation wiki](https://scp-wiki.wikidot.com) from terminal. 

## Installation

### With Rust compiler

If you have the Rust compiler installed (you can install it from [here](https://www.rust-lang.org/tools/install)), simply clone this repository and run
```
cargo build --release
```
inside the downloaded directory to compile the program. After that, you can run
```
cargo run --release -- <scp-code>
```
to run the application and browse the wiki page of the SCP indicated by \<scp-code\>. 

### Without Rust compiler

If you don't have the Rust compiler (or don't want to install it), you can extract `scp_terminal.exe` from `scp_terminal.zip` (which can be downloaded from [here](https://github.com/ultrapoci/scp_terminal/releases/)) in a folder of your choosing. Then, simply open your terminal, use the command `cd your\installation\directory\` to open the folder where you extracted `scp_terminal.exe` in your terminal and finally you can run
```
.\scp_terminal <scp-code>
```
to run the application. 

## Usage

### Example

```
cargo run --release -- 035
```
or 
```
.\scp_terminal 035
```
will show [SCP-035](https://scp-wiki.wikidot.com/scp-035)'s page. Note that at the moment the application is sensible to trailing zeros, meaning that "35" in the example above won't work. 

### Navigation

You can use the up or down arrow keys to navigate through the page, or you can use the PgUp and PgDown keys to navigate quicker. Press any other key to close the application.

### Help

You can run
```
cargo run --release -- --help
```
or
```
.\scp_terminal --help
```
to see the help page:
```
SCP Terminal 0.1.0
Use the Up or Down arrow keys to move through the page. Use PgUp or PgDown to move an entire page up or down. Press any other key to quit

USAGE:
    scp_terminal.exe <scp>

FLAGS:
    -h, --help
            Prints help information

    -V, --version
            Prints version information


ARGS:
    <scp>
            SCP code

            Show the page related to this SCP
```

## Note

This is a very early release. Expect bugs or strange behaviours. In particular, SCP pages from recent series tend to use a lot of gimmicks in their web page that makes it difficult to render them correctly in terminal. But this application should run fine with the first series of SCPs. 

## Suggestions for best experience

Here's how you can make reading the wiki through terminal more appealing. 
1. If you're on Windows 10, download the new [Windows Terminal](https://www.microsoft.com/it-it/p/windows-terminal/9n0dx20hk701?rtc=1&activetab=pivot:overviewtab). This is simply a better terminal than the default one and is safely available on Microsoft Store.
2. Open the new Windows Terminal, click the arrow above (near the tabs) and click on settings.
3. Here you can modify the behaviour of various shells. Add a new profile by  cloning Windows PowerShell. 
4. Open the settings for the newly created shell by clicking "Windows PowerShell (cloned)" on the left. 
5. On the right you will see various settings. Click on the second tab (it should be called "Appeareance") and switch on "Retro terminal effects". You can also increase the font size a bit to make it more readable.
   
Now you can use SCP Terminal from this Windows PowerShell modified to look like an old terminal from the 70's!.

![image](https://user-images.githubusercontent.com/50493113/127180878-76c2de05-b8a5-4680-bddf-006c348b7f9c.png)

