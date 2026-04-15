use fuse_mt::FileAttr;
use std::io;
use std::error;
pub struct BlockPoiner{
	blockPosition : u32,
	blockType : FileAttr
}



// all blocks are 512 bytes long

// holds metadata for a directory
pub struct DirectoryStartBlock{
	hash : [u8; 32],
	blockPosition : u32,
	name : [u8; 256],
	uid : u32,
	gid : u32,
	permissions : u32 ,
	directoryDataBlockPtr : u32,
	directoryDataBlockHash : [u8; 32],
	directoryBlockSize : u32,
}


// holds directory data
pub struct DirectoryDataBlock{
	hash : [u8; 32],
	blockPosition : u32,
	blockPointers : [BlockPoiner; 12],
	nextDirectoryDataBlockPos : u32,
	packing : [u8; 12]
}
pub struct StartBlock {
	hash : [u8; 32],
	blockPosition : u32,
	name : [u8; 256],
	attributes : FileAttr,
	//attributes is 70 bytes long
	firstDataBlockPos : u32,
	firstDataBlockHash : [u8; 32],
	lastDataBlockPos : u32,
	lastDataBlockHash : [u8; 32],
	padding : [u8; 82]

}


pub struct FileDataBlock {
	hash : [u8; 32],
	blockPosition : u32,
	data : [u8; 472],
	nextDataBlockPos : u32,
}

trait GenericBlock{
	fn GetPosition() -> u32{
		return 2;
	}
}


