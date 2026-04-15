

- blocks will be 512 bytes long
- all blocks will hold the SHA 256 hash of contents
- there will be 3 block types

	- directory
		- will hold pointers to file start blocks and will hold pointers to other directory blocks with the final section being deticated to a pointer either to itself or to the next directiory block fo the current directiory
	- directory start
		- will hold the name and metadata for a directory

	- file start
		- will hold the metadata for a file like last modification, creation time, name, hash of the total data, pointer to the first data block, the GID, the UID, and the unique file ID

	- file Data
		- will hold file data alongside a hash of all of the data and will hold a pointer to the next file data block or back to the file start block if it is the final data block for a file




