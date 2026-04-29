use fuse_mt::{FileAttr,FileType,DirectoryEntry};

use std::ffi::OsString;
//use std::io;
//e std::error;
use std::time::SystemTime;
use super::driveActions::{data_block_read,start_block_read};
use std::fs::File;
use super::handle::FileHandle;
pub struct BlockPoiner{
	blockPosition : u32,
}

pub enum Types {
	NamedPipe   = 0b00000001,
    CharDevice  = 0b00000010,
    BlockDevice = 0b00000100,
    Directory   = 0b00001000,
    RegularFile = 0b00010000,
    Symlink     = 0b00100000, 
    Socket      = 0b01000000,
}


// 
// pub struct FileAttr {Show 13 fields
//     pub size: u64,
//     pub blocks: u64,
//     pub atime: SystemTime,
//     pub mtime: SystemTime,
//     pub ctime: SystemTime,
//     pub crtime: SystemTime,
//     pub kind: FileType,
//     pub perm: u16,
//     pub nlink: u32,
//     pub uid: u32,
//     pub gid: u32,
//     pub rdev: u32,
//     pub flags: u32,
// }


// 
//     NamedPipe,
//     CharDevice,
//     BlockDevice,
//     Directory,
//     RegularFile,
//     Symlink,
//     Socket,
impl From<MetaData> for FileAttr{
	fn from(data_in : MetaData) -> Self {
		FileAttr{
			size : data_in.size,
			blocks : data_in.blockLen,
			atime: SystemTime::UNIX_EPOCH,
			ctime: SystemTime::UNIX_EPOCH,
			mtime: SystemTime::UNIX_EPOCH,
			crtime: SystemTime::UNIX_EPOCH,
			perm: data_in.perm,
			uid : data_in.uid,
			gid : data_in.gid,
			kind : match data_in.fileType{
			1 => FileType::NamedPipe,
			2 => FileType::CharDevice,
			4 => FileType::BlockDevice,
			8 => FileType::Directory,
			16 => FileType::RegularFile,
			32 => FileType::Symlink, 
			64 => FileType::Socket,
			_ => FileType::RegularFile
			},
			nlink : 1,
			rdev : 0,
			flags : 0
		}
		
	}
}

#[derive(Clone)]
pub struct MetaData {
	pub size : u64,
	pub blockLen : u64,
	pub aTime : u128,
	pub mTime : u128,
	pub cTime : u128,
	pub perm : u16,
	pub uid : u32,
	pub gid : u32,
	pub fileType : u8, 
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
// pub struct DirectoryDataBlock{
// 	pub hash : [u8; 32],
// 	blockPosition : u32,
// 	blockTypeId : u32,
// 	blockPointers : [u32; 117],
// 	nextDirectoryDataBlockPos : u32,
// }







// it is the same data for both dir and files
#[derive(Clone)]
pub struct StartBlock {
	pub hash : [u8; 32], 			// 000 - 01F
	pub blockPosition : u32, 		// 020 - 023
	pub blockTypeId : u32,				// 024 - 027
	name : [u8; 128],				// 038 - 0A7
	pub attributes : MetaData,		// 0A8 - 0F2
	// padding 						// 0F3 - 0FF
	pub firstDataBlockPos : u32,	// 100 - 104
	// padding 						// 105 - 10F
	firstDataBlockHash : [u8; 32],	// 110 - 12F
	lastDataBlockPos : u32,			// 130 - 134
	// padding						// 135 - 13F
	lastDataBlockHash : [u8; 32],	// 140 - 15F
	// padding						// 160 - 1FF
}

impl From<StartBlock> for RawBlock{
	fn from(in_block : StartBlock) -> Self{
		let mut data_vec : Vec<u8> = Vec::with_capacity(512);
		for byte in in_block.hash{
			data_vec.push(byte);
		}
		for byte in in_block.blockPosition.to_le_bytes(){
			data_vec.push(byte);
		}
		for byte in in_block.blockTypeId.to_le_bytes(){
			data_vec.push(byte);
		}
		for byte in in_block.name{
			data_vec.push(byte);
		}
		for byte in in_block.attributes.size.to_le_bytes(){
			data_vec.push(byte);
		}
		for byte in in_block.attributes.blockLen.to_le_bytes(){
			data_vec.push(byte);
		}
		for byte in in_block.attributes.uid.to_le_bytes(){
			data_vec.push(byte);
		}
		for byte in in_block.attributes.gid.to_le_bytes(){
			data_vec.push(byte);
		}
		for byte in in_block.attributes.aTime.to_le_bytes(){
			data_vec.push(byte);
		}
		for byte in in_block.attributes.mTime.to_le_bytes(){
			data_vec.push(byte);
		}		
		for byte in in_block.attributes.cTime.to_le_bytes(){
			data_vec.push(byte);
		}
		for byte in in_block.attributes.perm.to_le_bytes(){
			data_vec.push(byte);
		}
		for byte in in_block.attributes.fileType.to_le_bytes(){
			data_vec.push(byte);
		}
		for _i in 0..13{
			data_vec.push(0);
		}
		for byte in in_block.firstDataBlockPos.to_le_bytes(){
			data_vec.push(byte);
		}
		for _i in 0..12{
			data_vec.push(0);
		}
		for byte in in_block.firstDataBlockHash{
			data_vec.push(byte);
		}
		for byte in in_block.firstDataBlockPos.to_le_bytes(){
			data_vec.push(byte);
		}
		for _i in 0..12{
			data_vec.push(0);
		}
		for byte in in_block.firstDataBlockHash{
			data_vec.push(byte);
		}
		for _i in 0..160{
			data_vec.push(0);
		}
		let mut data_arr : [u8;512] = [0;512];
		for i in 0..512{
			data_arr[i] = data_vec[i];
		}
		RawBlock{
			data : data_arr,
		}
	}
}



impl From<RawBlock> for StartBlock{
	fn from(in_block : RawBlock) -> Self{
		let data = in_block.data;
		StartBlock{
			hash : 				<[u8;32]>::try_from(&data[0..=31]).expect("the data array is haveing issues"),
			blockPosition : 	u32::from_le_bytes(<[u8;4]>::try_from(&data[32 ..= 35]).unwrap()),
			blockTypeId :		u32::from_le_bytes(<[u8;4]>::try_from(&data[36 ..= 39]).unwrap()),
			name : 				<[u8; 128]>::try_from(&data[40..=167]).unwrap(),
			// Metadata 168..=274	
			attributes:			MetaData {
				size :			u64::from_le_bytes(<[u8;8]>::try_from(&data[168 ..= 175]).unwrap()),
				blockLen :		u64::from_le_bytes(<[u8;8]>::try_from(&data[176 ..= 183]).unwrap()),
				uid : 			u32::from_le_bytes(<[u8;4]>::try_from(&data[184 ..= 187]).unwrap()),
				gid :			u32::from_le_bytes(<[u8;4]>::try_from(&data[188 ..= 191]).unwrap()),
				aTime : 		u128::from_le_bytes(<[u8;16]>::try_from(&data[192 ..= 207]).unwrap()),
				mTime : 		u128::from_le_bytes(<[u8;16]>::try_from(&data[208 ..= 223]).unwrap()),
				cTime : 		u128::from_le_bytes(<[u8;16]>::try_from(&data[224 ..= 239]).unwrap()),
				perm :			u16::from_le_bytes(<[u8;2]>::try_from(&data[240 ..= 241]).unwrap()),
				fileType : 		data[242]
			},
			firstDataBlockPos:	u32::from_le_bytes(<[u8;4]>::try_from(&data[256 ..= 259]).unwrap()),
			firstDataBlockHash:	<[u8;32]>::try_from(&data[272..=303]).expect("the data array is haveing issues"),
			lastDataBlockPos:	u32::from_le_bytes(<[u8;4]>::try_from(&data[304 ..= 307]).unwrap()),
			lastDataBlockHash:	<[u8;32]>::try_from(&data[320..=351]).expect("the data array is haveing issues")

		}
	}
}

impl From<StartBlock> for RawDataBlock{
	fn from(in_block : StartBlock) -> Self{
		let mut data_vec : Vec<u8> = Vec::with_capacity(472);
		for byte in in_block.name{
			data_vec.push(byte);
		}
		for byte in in_block.attributes.size.to_le_bytes(){
			data_vec.push(byte);
		}
		for byte in in_block.attributes.blockLen.to_le_bytes(){
			data_vec.push(byte);
		}
		for byte in in_block.attributes.uid.to_le_bytes(){
			data_vec.push(byte);
		}
		for byte in in_block.attributes.gid.to_le_bytes(){
			data_vec.push(byte);
		}
		for byte in in_block.attributes.aTime.to_le_bytes(){
			data_vec.push(byte);
		}
		for byte in in_block.attributes.mTime.to_le_bytes(){
			data_vec.push(byte);
		}		
		for byte in in_block.attributes.cTime.to_le_bytes(){
			data_vec.push(byte);
		}
		for byte in in_block.attributes.perm.to_le_bytes(){
			data_vec.push(byte);
		}
		for byte in in_block.attributes.fileType.to_le_bytes(){
			data_vec.push(byte);
		}
		for _i in 0..13{
			data_vec.push(0);
		}
		for byte in in_block.firstDataBlockPos.to_le_bytes(){
			data_vec.push(byte);
		}
		for _i in 0..12{
			data_vec.push(0);
		}
		for byte in in_block.firstDataBlockHash{
			data_vec.push(byte);
		}
		for byte in in_block.firstDataBlockPos.to_le_bytes(){
			data_vec.push(byte);
		}
		for _i in 0..12{
			data_vec.push(0);
		}
		for byte in in_block.firstDataBlockHash{
			data_vec.push(byte);
		}
		for _i in 0..160{
			data_vec.push(0);
		}
			
		let mut data_arr : [u8;472] = [0;472];
		for i in 0..472{
			data_arr[i] = data_vec[i];
		}

		RawDataBlock{
			hash: in_block.hash,
			blockPosition: in_block.blockPosition,
			blockTypeId: in_block.blockTypeId,
			data: data_arr
		}
		
		
	}
}




#[derive(Clone)]
pub struct DataBlock {
	pub hash : [u8; 32],			// 000 - 01F
	// padding						// 028 - 02F
	pub blockPosition : u32,		// 020 - 023
	blockTypeId : u32,				// 024 - 027
	pub data : [u8; 448],				// 030 - 1EF
	//padding						// 1F0 - 1FB
	pub nextDataBlockPos : u32,		// 1FC - 1FF
}


impl From<RawBlock> for DataBlock{
	fn from(in_block : RawBlock) -> Self{
		let data = in_block.data;
		DataBlock{
			hash : 				<[u8;32]>::try_from(&data[0..=31]).expect("the data array is haveing issues"),
			blockPosition : 	u32::from_le_bytes(<[u8;4]>::try_from(&data[32 ..= 35]).expect("block pos is not parseing correct")),
			blockTypeId :		u32::from_le_bytes(<[u8;4]>::try_from(&data[36 ..= 39]).expect("block type ID is not parseing correct")),
			data : 				<[u8;448]>::try_from(&data[48..=495]).expect("the data array is haveing issues"),
			nextDataBlockPos :	u32::from_le_bytes(<[u8;4]>::try_from(&data[508 ..= 511]).expect("next block pos is not parseing correct"))
		}
	}
}

impl From<DataBlock> for RawBlock{
	fn from(in_block: DataBlock)-> Self{
		let mut data_vec : Vec<u8> = Vec::with_capacity(512);
		for byte in in_block.hash{
			data_vec.push(byte);
		}
		for byte in in_block.blockPosition.to_le_bytes(){
			data_vec.push(byte);
		}
		for byte in in_block.blockTypeId.to_le_bytes(){
			data_vec.push(byte);
		}
		for _i in 0..8{
			data_vec.push(0);
		}
		for byte in in_block.data{
			data_vec.push(byte);
		}
		for _i in 0..12{
			data_vec.push(0);
		}
		for byte in in_block.nextDataBlockPos.to_le_bytes(){
			data_vec.push(byte);
		}

		let mut data_arr : [u8;512] = [0;512];
		for i in 0..512{
			data_arr[i] = data_vec[i];
		}
		RawBlock{
			data : data_arr,
		}
		
		
	}
}


impl From<DataBlock> for RawDataBlock{
	fn from(in_block : DataBlock) -> Self{
		let mut data_vec : Vec<u8> = Vec::with_capacity(472);
		for _i in 0..8{
			data_vec.push(0);
		}
		for byte in in_block.data{
			data_vec.push(byte);
		}
		for _i in 0..12{
			data_vec.push(0);
		}
		for byte in in_block.nextDataBlockPos.to_le_bytes(){
			data_vec.push(byte);
		}
		let mut data_arr : [u8;472] = [0;472];
		for i in 0..472{
			data_arr[i] = data_vec[i];
		}
		RawDataBlock{
			hash: in_block.hash,
			blockPosition: in_block.blockPosition,
			blockTypeId: in_block.blockTypeId,
			data: data_arr
		}
				
	}
}



//defineing methods and traits
pub trait GenericBlock {
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
	pub fn get_data_start_pos(&self) -> u32{
		self.firstDataBlockPos
	}
	pub fn get_first_data_block_hash(&self) -> [u8; 32]{
		self.firstDataBlockHash
	}
	pub fn check_first_data_block_hash(&self) -> bool{
		//TODO implement this
		true
	}

	pub fn get_name(&self) -> OsString{
		if self.blockPosition == 0{
			return OsString::from("/")
		}
		let unparsed_name = self.name.to_vec();
		let mut magic_vec : Vec<u8> = Vec::with_capacity(247);
		for i in unparsed_name{
			if i != 0{
				magic_vec.push(i);
			}
		}



		
		return OsString::from(String::from_utf8(magic_vec).unwrap())
	}
	
	pub fn set_attributes(&mut self, newAttr : MetaData){
		self.attributes = newAttr
	}
	pub fn get_attributes(self) -> MetaData{
		self.attributes
	}

	pub fn new(hash : [u8; 32],
		blockPosition : u32,
		blockTypeId : u32,
		name : [u8; 128],
		attributes : MetaData,
		firstDataBlockPos : u32,
		firstDataBlockHash : [u8; 32],
		lastDataBlockPos : u32,
		lastDataBlockHash : [u8; 32],
		) -> Self{
			StartBlock{hash ,
				blockPosition,
				blockTypeId,
				name ,
				attributes ,
				//attributes is 76 bytes long
				firstDataBlockPos,
				firstDataBlockHash,
				lastDataBlockPos,
				lastDataBlockHash,
			}
		}
}




impl MetaData{
	pub fn new(
	size : u64,
	blockLen : u64,
	aTime : u128,
	mTime : u128,
	cTime : u128,
	perm : u16,
	uid : u32,
	gid : u32,
	fileType : u8, ) -> Self{
		Self{size,
			blockLen ,
			aTime ,
			mTime ,
			cTime ,
			perm ,
			uid ,
			gid ,
			fileType }
		
	}






}

//generic block implementaions



impl StartBlock{
	pub fn get_block_pos(&self) -> u32{
		self.blockPosition
	}
	pub fn get_block_hash(&self) -> [u8;32]{
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




// Data Block implemetations

impl DataBlock{
	pub fn get_next_block_pos(&self) -> u32{
		self.nextDataBlockPos
	}
	pub fn new(
		hash : [u8; 32],
		blockPosition : u32,
		blockTypeId : u32,
		data : [u8; 448],
		nextDataBlockPos : u32,) -> Self{
			DataBlock{
				hash,
				blockPosition,
				blockTypeId,
				data,
				nextDataBlockPos
			}
		}


	pub fn set_data(&mut self, data : [u8;448]){
		self.data = data;
	} 


	pub fn set_next_block_pos(&mut self,pos : u32){
		self.nextDataBlockPos = pos;
	}

	// This will output a vector of all of the start blocks pointed to by the data in a data block
	pub fn parse_to_directory_ptrs(&self, file : &File) -> Vec<u32>{
		let mut ptr_vec : Vec<u32> = Vec::with_capacity(112);
		let mut buffer_for_bytes : [u8;4] = [0;4];
		let data = self.data;
		let mut data_idr = data.into_iter();
		let mut perhaps_number : u32;
		for _i in 0..112{
			buffer_for_bytes[0] = data_idr.next().expect("issue1");
			buffer_for_bytes[1] = data_idr.next().expect("issue2");
			buffer_for_bytes[2] = data_idr.next().expect("issue2");
			buffer_for_bytes[3] = data_idr.next().expect("issue2");
			perhaps_number = u32::from_le_bytes(buffer_for_bytes);
			if perhaps_number != 0{
				ptr_vec.push(perhaps_number);
			}
		}
		
		
		// if self.nextDataBlockPos != u32::MAX {
		// 	ptr_vec.append(&mut data_block_read(file,self.nextDataBlockPos).parse_to_directory_ptrs(file));
		// }
		 
		ptr_vec
	}




	pub fn find_directory_entry_slot(&self, start_block_index_in_question : u32) -> u32{
		let mut ptr_vec : Vec<u32> = Vec::with_capacity(112);
		let mut buffer_for_bytes : [u8;4] = [0;4];
		let data = self.data;
		let mut data_idr = data.into_iter();
		let mut perhaps_number : u32;
		let mut current_slot = u32::MAX;
		for i in 0..112{
			buffer_for_bytes[0] = data_idr.next().expect("issue1");
			buffer_for_bytes[1] = data_idr.next().expect("issue2");
			buffer_for_bytes[2] = data_idr.next().expect("issue2");
			buffer_for_bytes[3] = data_idr.next().expect("issue2");
			perhaps_number = u32::from_le_bytes(buffer_for_bytes);
			if perhaps_number == start_block_index_in_question{
				current_slot = i;
				break;
			}else {
				continue;
			
			}
		}
		return current_slot;
		
	}

	pub fn parse_to_full_data(&self, file : &File) -> Vec<u8>{
		let mut data_vec : Vec<u8> = Vec::with_capacity(1028);
		data_vec.append(&mut self.data.to_vec());
		if data_vec.is_empty(){
			data_vec.push(0);
		}
		if self.nextDataBlockPos != 0 {
			data_vec.append(&mut data_block_read(&file,self.nextDataBlockPos).parse_to_full_data(file));
		}
		data_vec
	}

	// my function names just roll of the tongue
	pub fn get_data_block_pos_from_block_offset(&self, file : &File, block_offset : u32) -> u32{
		if block_offset == 0{
			self.blockPosition
		} else {
			if self.nextDataBlockPos == 0{
				0
			}else {
				return data_block_read(&file, self.nextDataBlockPos).get_data_block_pos_from_block_offset(&file,block_offset-1)
			}
		}
	}
	pub fn get_data(self) -> [u8;448] {
		return self.data
	}



	pub fn parse_directory_to_directory_entry_struct_vector(&self, file : &File, _handle : FileHandle) -> Vec<DirectoryEntry>{
		let directory_ptr_vec = self.parse_to_directory_ptrs(file);
		let mut directory_entry_vec : Vec<DirectoryEntry> = Vec::with_capacity(112);

		for ptr in directory_ptr_vec{

			let block_in_question = start_block_read(&file, ptr);

			let parsed_name = block_in_question.name.to_vec().extract_if(..,|x| *x != 0).collect::<Vec<_>>();
			directory_entry_vec.push(
					DirectoryEntry{
						name : OsString::from(String::from_utf8(parsed_name).unwrap()),
						kind : match block_in_question.attributes.fileType {
									0b00000001 => FileType::NamedPipe,
									0b00000010 => FileType::CharDevice,
									0b00000100 => FileType::BlockDevice,
									0b00001000 => FileType::Directory,
									0b00010000 => FileType::RegularFile,
									0b00100000 => FileType::Symlink, 
									0b01000000 => FileType::Socket,
									_ => FileType::Directory
								}
					}
				)
			}
			directory_entry_vec
		}
	}
