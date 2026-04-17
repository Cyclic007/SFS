//use fuse_mt::FileAttr;
//use std::io;
//e std::error;
pub struct BlockPoiner{
	blockPosition : u32,
}
// pub enum Blocks{
// 	DirectoryDataBlock = "DIR",
// 	StartBlock = "STR",
// 	FileDataBlock = "FIL"
// }

pub enum Types {
	NamedPipe   = 0x00000001,
    CharDevice  = 0x00000010,
    BlockDevice = 0x00000100,
    Directory   = 0x00001000,
    RegularFile = 0x00010000,
    Symlink     = 0x00100000, 
    Socket      = 0x01000000,
}


pub struct MetaData {
	size : u64,
	blockLen : u64,
	aTime : u128,
	mTime : u128,
	cTime : u128,
	perm : u16,
	uid : u32,
	gid : u32,
	fileType : u8, 
}






//these stucts are for io handleing
pub struct RawBlock{
	pub data : [u8; 512]
}
pub struct RawDataBlock{
	pub	hash : [u8; 32],
	pub	blockPosition : u32,
	pub blockTypeId : u32,
	pub data : [u8; 472],
}


// all blocks are 512 bytes long

// holds metadata for a directory

// holds directory data
pub struct DirectoryDataBlock{
	hash : [u8; 32],
	blockPosition : u32,
	blockTypeId : u32,
	blockPointers : [u32; 117],
	nextDirectoryDataBlockPos : u32,
}


// it is the same data for both dir and files
pub struct StartBlock {
	hash : [u8; 32],
	blockPosition : u32,
	blockTypeId : u32,
	name : [u8; 247],
	attributes : MetaData,
	//attributes is 76 bytes long
	firstDataBlockPos : u32,
	firstDataBlockHash : [u8; 32],
	lastDataBlockPos : u32,
	lastDataBlockHash : [u8; 32],
	padding : [u8; 81]

}


pub struct DataBlock {
	hash : [u8; 32],
	blockPosition : u32,
	blockTypeId : u32,
	data : [u8; 468],
	nextDataBlockPos : u32,
}






//defineing methods and traits
trait GenericBlock {
	fn get_block_pos(&self) -> u32;
	fn get_block_hash(&self) -> [u8;32];
	fn check_hash(&self) -> bool;
}


trait DataBlocks {
	fn get_next_block_pos(&self) -> u32;
}

trait FileData {
	fn get_data(&self) -> [u8;472];
	fn set_data(&mut self, newData : [u8;472]);
}

trait DirectoryData{
	fn get_contents_ptrs(&self) -> [u32; 118];
}








impl StartBlock {
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
	
	fn set_attributes(&mut self, newAttr : MetaData){
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

impl GenericBlock for DataBlock{
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

// extracting generic headers from a raw block
impl From<RawBlock> for RawDataBlock{
	fn from(inBlock : RawBlock) -> Self{
		let data = inBlock.data;

		let pos_bytes = match <[u8; 4]>::try_from(&data[33..37]) {
	        Ok(array) => array,
	        Err(_e) => panic!("AHHHHHHHHHHHHHHHHHHH")
	    };
	    let id_bytes = match <[u8; 4]>::try_from(&data[37..41]) {
   	        Ok(array) => array,
   	        Err(_e) => panic!("AHHHHHHHHHHHHHHHHHHH")
   	    };

		RawDataBlock{
			hash : <[u8; 32]>::try_from(&data[..32]).unwrap(),
			blockPosition : u32::from_le_bytes(pos_bytes),
			blockTypeId : u32::from_le_bytes(id_bytes),
			data : <[u8; 472]>::try_from(&data[40..]).unwrap(),
		}
	}
}






// going from raw with headers to actual block
impl From<RawDataBlock> for DataBlock{
	fn from(inBlock : RawDataBlock ) -> Self{
		let data = inBlock.data;
		
		DataBlock {
			hash : inBlock.hash,
			blockPosition : inBlock.blockPosition,
			blockTypeId : inBlock.blockTypeId,
			data : <[u8; 468]>::try_from(&data[..468]).unwrap(),
			nextDataBlockPos : u32::from_le_bytes(<[u8; 4]>::try_from(&data[468..]).unwrap()),
		}	
	}	
}

impl From<RawDataBlock> for StartBlock{
	fn from(inBlock : RawDataBlock ) -> Self{
		let data = inBlock.data;

		StartBlock {
			hash : inBlock.hash,
			blockPosition : inBlock.blockPosition,
			blockTypeId : inBlock.blockTypeId,
			name : <[u8; 247]>::try_from(&data[..247]).unwrap(),
			attributes : MetaData{
				size : u64::from_le_bytes(<[u8; 8]>::try_from(&data[247..255]).unwrap()),
				blockLen : u64::from_le_bytes(<[u8; 8]>::try_from(&data[255..263]).unwrap()),
				aTime : u128::from_le_bytes(<[u8; 16]>::try_from(&data[263..279]).unwrap()),
				mTime : u128::from_le_bytes(<[u8; 16]>::try_from(&data[279..295]).unwrap()),
				cTime : u128::from_le_bytes(<[u8; 16]>::try_from(&data[295..311]).unwrap()),
				perm : u16::from_le_bytes(<[u8; 2]>::try_from(&data[311..313]).unwrap()),
				uid : u32::from_le_bytes(<[u8; 4]>::try_from(&data[313..317]).unwrap()),
				gid : u32::from_le_bytes(<[u8; 4]>::try_from(&data[317..321]).unwrap()),
				fileType : inBlock.data[321],
			},
			firstDataBlockHash : <[u8; 32]>::try_from(&data[322..354]).unwrap(),
			firstDataBlockPos : u32::from_le_bytes(<[u8; 4]>::try_from(&data[354..358]).unwrap()),
			lastDataBlockHash : <[u8; 32]>::try_from(&data[358..390]).unwrap(), 
			lastDataBlockPos : u32::from_le_bytes(<[u8; 4]>::try_from(&data[390..394]).unwrap()),

			padding : <[u8; 81]>::try_from(&data[394..=471]).unwrap(),
		}
	}	
}


// impl From<RawDataBlock> for DirectoryDataBlock{
// 	fn from(inBlock : RawDataBlock ) -> Self{
// 		let data = inBlock.data;
// 		DirectoryDataBlock{
// 			hash : inBlock.hash,
// 			blockPosition : inBlock.blockPosition,
// 			blockTypeId : inBlock.blockTypeId,
// 			blockPointers : <[u32; 117]>::try_from(<[u8;468]>::try_from(&data[..468]).unwrap()).unwrap(),
// 			nextDataBlockPos : u32::from_le_bytes(<[u8; 4]>::try_from(&data[468..]).unwrap()),
// 		}				
// 	}
// 	
// }
	// hash : [u8; 32],
	// blockPosition : u32,
	// blockTypeId : u32,
	// blockPointers : [u32; 117],
	// nextDirectoryDataBlockPos : u32,




// Data Block implemetations
impl DataBlocks for DataBlock{
	fn get_next_block_pos(&self) -> u32{
		self.nextDataBlockPos
	}
}



// 
// impl DirectoryData for DirectoryDataBlock{
// 	fn get_contents_ptrs(&self) -> [u32; 118] {
// 		self.blockPointers
// 	}
// 	
// }




// impl RawDataBlock {
// 	pub fn get_data(self) -> [u8; 472]{
// 		self.data
// 	}	
// }
// 
// 
// impl RawBlock {
// 	pub fn get_data(self) -> [u8; 512]{
// 		self.data
// 	}
// }



