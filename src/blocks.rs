use fuse_mt::FileAttr;
use std::io;
use std::error;
pub struct BlockPoiner{
	blockPosition : u32,
}
pub enum Blocks{
	DirectoryDataBlock,
	StartBlock,
	FileDataBlock
}


pub struct RawBlock{
	data : [u8; 512]
}



// all blocks are 512 bytes long

// holds metadata for a directory

// holds directory data
pub struct DirectoryDataBlock{
	hash : [u8; 32],
	blockPosition : u32,
	blockPointers : [u32; 118],
	nextDirectoryDataBlockPos : u32,
}


// it is the same data for both dir and files
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




//defineing methods and traits
trait GenericBlock {
	fn get_block_pos(&self) -> u32;
	fn get_block_hash(&self) -> [u8;32];
	fn check_hash(&self) -> bool;
}


trait DataBlock {
	fn get_next_block_pos(&self) -> u32;
	fn get_data_block_type(&self) -> Blocks;
}

trait FileData {
	fn get_data(&self) -> [u8;472];
	fn set_data(&mut self, newData : [u8;472]);
}

trait DirectoryData{
	fn get_contents_ptrs(&self) -> [u32; 118];
}

impl StartBlock {
	fn get_name(&self) -> [u8; 256]{
		self.name
	}
	fn get_data_start_pos(&self) -> u32{
		self.firstDataBlockPos
	}
	fn get_first_data_block_hash(&self) -> [u8; 32]{
		self.firstDataBlockHash
	}
	fn check_first_data_block_hash(&self) -> bool{
		//TODO implement this
		true
	}
	fn get_attributes(&self) -> FileAttr{
		self.attributes
	}
	fn set_attributes(&mut self, newAttr : FileAttr){
		self.attributes = newAttr
	}
}








//generic block implementaions

impl GenericBlock for DirectoryDataBlock{
	fn get_block_pos(&self) -> u32{
		self.blockPosition
	}
	fn get_block_hash(&self) -> [u8;32]{
		self.hash
	}
	fn check_hash(&self) -> bool{
		//TODO implement hash checks
		true
	}
}


impl GenericBlock for StartBlock{
	fn get_block_pos(&self) -> u32{
		self.blockPosition
	}
	fn get_block_hash(&self) -> [u8;32]{
		self.hash
	}
	fn check_hash(&self) -> bool{
		//TODO implement hash checks
		true
	}
}

impl GenericBlock for FileDataBlock{
	fn get_block_pos(&self) -> u32{
		self.blockPosition
	}
	fn get_block_hash(&self) -> [u8;32]{
		self.hash
	}

	fn check_hash(&self) -> bool{
		//TODO implement hash checks
		true
	}
}






// Data Block implemetations
impl DataBlock for FileDataBlock{
	fn get_next_block_pos(&self) -> u32{
		self.nextDataBlockPos
	}
	fn get_data_block_type(&self) -> Blocks{
		Blocks::FileDataBlock
	}
}


impl DataBlock for DirectoryDataBlock{
	fn get_next_block_pos(&self) -> u32{
		self.nextDirectoryDataBlockPos
	}
	fn get_data_block_type(&self) -> Blocks{
		Blocks::DirectoryDataBlock
	}
}


impl DirectoryData for DirectoryDataBlock{
	fn get_contents_ptrs(&self) -> [u32; 118] {
		self.blockPointers
	}
	
}
