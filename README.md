# SFS
Secure File system

This is a FUSE file system that I made as my first real rust project

this was inspired by DocJade 





# Systems

this File System uses 512 byte blocks to store data where each block contains a hash of it's contents



# how to run

## This will not work on windows without lots of fiddleing around with WSL

### you will need to run as root

- you run this command inside of the repo
'cargo run \<the device or vitual image path\> \<the mount path\>'

- or you run the executible with the same arguments 

- and that will mount the file system to your device at the specified mount point and hold your terminal hostage

- to interact with the file system you use a seprate terminal

- a quark of how this works means that the mount point becomes owned by root so you need to be root to run commands inside unless you chmod or chown the mount point directory

--- 

### this file system does not support
- links
- renaming files
- TTL
- seting modification times to files

