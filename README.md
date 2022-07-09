# mr3-advanced-viewer v0.0.2(prototype)
A GUI application to view Monster Rancher 3 data running in pcsx2.
# WARNING!!!
This only works with PCSX 1.6.0 currently, as the most recent Nightly builds, and probably other more recent versions of PCSX2,have done something which changes the memory locations of all the data in the game. PCSX2's official build as of writing this is currently 1.6.0, so just go to their site and download that if the iewer isn't working.

# How to Run
## Windows
Just select and run the executable. After that, if you already have MR3 running, just hit connect.

Currently, it does NOT update automatically. You have to hit connect everytime the game updates the values(AKA: hit connect every week).

For reporting bugs, DM or ping in any MR discord @most#4673.



# Compilation
## Windows
Go to https://www.rust-lang.org/tools/install and install the version you need (most people need the 64-bit one) and run it. At the install prompt, just hit y and enter until finished.

After that, make sure you have Visual C++ installed and up-to-date, and then go to https://www.visualstudio.microsoft.com/downloads/  and install the Build Tools for Visual Studio. On its installation page, select Universal Windows  Build Tools, Desktop Development with C++, and .NET desktop build tools. After that, go into the command prompt and type:
```cd your\dir\name\here\```

When you type dir into the command prompt, you should see Cargo.toml. If you do, just type cargo build to build an executable in the target/debug directory!


