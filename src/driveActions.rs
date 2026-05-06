use std::{
    fs::File,
    os::unix::fs::FileExt,
    collections::VecDeque,
	ffi::{OsStr,OsString}
};

use super::blocks::{RawDataBlock,RawBlock,StartBlock,DataBlock,MetaData};

use sha_256::Sha256;
// these are low level and should not be used mutch
// THIS DOES NOT CHECK THE HASH OR THE BLOCK POS
pub fn direct_block_read(
	driveFile : &File,
	blockIndex : u32,
) -> RawBlock {
	let mut read_buff : [u8; 512] = [0;512];
	let mut sha256: Sha256 = Sha256::new();
	
	let read_offset : u64 = u64::from(blockIndex) * 512;
	let _read_result = driveFile.read_exact_at(&mut read_buff, read_offset);

	let raw_data_block = RawDataBlock::from(RawBlock{data :read_buff});
	if raw_data_block.hash != sha256.digest(&raw_data_block.data) || raw_data_block.hash != [0;32]{
		println!("this file has been tampered with");
	}

	RawBlock{data : read_buff}
}


pub fn direct_block_write(
	driveFile : &File,
	block : RawDataBlock,
	blockIndex : u32)							{
	//first we write the hash and index
	
	//prep for hash
	let mut sha256: Sha256 = Sha256::new();
	//write the block pos
//	block.blockPosition = blockIndex;
	let data = &block.data;

	//add the hash
	let hash = sha256.digest(data);
	let Type : u32  = block.blockTypeId.clone();
	let mut output : [u8; 512] = [0;512];

	for i in 0..32{
		output[i] = hash[i];
	}
	let index : [u8; 4] = u32::to_ne_bytes(blockIndex);

	for i in 0..4{
		output[i+32] = index[i];
	}
	
	let typeID : [u8; 4] = u32::to_ne_bytes(Type);

	for i in 0 .. 4 {
		output[i+36] = typeID[i];
	}



	for i in 0 ..472{
		output[i+40] = data[i];
	}
	//write the block to drive
	let write_offset : u64 = u64::from(blockIndex)*512;
	driveFile.write_all_at(&output,write_offset);	
}

pub fn direct_block_delete(drive_file : &File,	block_index: u32) {
	let output : [u8; 512] = [0;512];
	let write_offset : u64 = u64::from(block_index)*512;

	drive_file.write_all_at(&output,write_offset);
}







fn start_block_write(driveFile : &File,block :StartBlock,blockIndex : u32){
	direct_block_write(&driveFile,RawDataBlock::from(RawBlock::from(block)),blockIndex)
}
fn data_block_write(driveFile : &File,block :DataBlock,blockIndex : u32){
	direct_block_write(&driveFile,RawDataBlock::from(block),blockIndex)
}



fn data_block_next_pos_modify(drive_file : &File, block_index: u32, new_pos : u32){
	let mut block = data_block_read(&drive_file.try_clone().unwrap(),block_index);

	block.nextDataBlockPos = new_pos;
	data_block_write(drive_file,block,block_index);
	
}



pub fn start_block_read(
	driveFile : &File,
	blockIndex : u32,
) -> StartBlock{
	StartBlock::from(direct_block_read(driveFile,blockIndex))	
}

pub fn data_block_read(
	driveFile : &File,
	blockIndex : u32,
) -> DataBlock{
	DataBlock::from(direct_block_read(driveFile,blockIndex))
}	


// this will return the start value for the chunk of data
pub fn data_write_new(driveFile : &File, mut data : VecDeque<u8>) -> u32{
	let i : u32= 0;
	let mut empty_block_indexes : Vec<u32> = vec!(0,0);
	empty_block_indexes.reserve(256);
	let data_size = data.clone().len();	
	let mut num_of_blocks = data_size/448;
	if data_size%448 != 0{
		num_of_blocks += 1;
	}


	

	loop {
		
		let checkBlock = direct_block_read(&driveFile, i);
		if RawDataBlock::from(checkBlock).hash == [0;32]{
			//This means that this block is empty
			empty_block_indexes.push(i);
			if empty_block_indexes.len() == num_of_blocks{
				break;
			}		
		}
	}


	loop {
		if let Some(block_index) = empty_block_indexes.pop() && empty_block_indexes.len() >= 2 {
			let mut data_into_current_block : [u8;472] = [0;472];
			let next_block_pos_bytes : [u8; 4] = empty_block_indexes[empty_block_indexes.len()-1].to_le_bytes();
			
			
			for i in 8..456{
				if let Some(current_byte) = data.pop_front(){
					data_into_current_block[i] = current_byte;
				}
			}
			
			data_into_current_block[0] = next_block_pos_bytes[0];
			data_into_current_block[1] = next_block_pos_bytes[1];
			data_into_current_block[2] = next_block_pos_bytes[2];
			data_into_current_block[3] = next_block_pos_bytes[3];

			direct_block_write(&driveFile,RawDataBlock{
				hash : [0; 32],
				blockPosition : block_index,
				blockTypeId : 0b000100000,
				data : data_into_current_block
			},block_index )
		} else {
			if let Some(block_index) = empty_block_indexes.pop(){
				let mut data_into_current_block : [u8;472] = [0;472];
				let next_block_pos_bytes : [u8; 4] = [0;4];

				for i in 8..456{
					if let Some(current_byte) = data.pop_front(){
						data_into_current_block[i] = current_byte;
					}
				}
				
				data_into_current_block[0] = next_block_pos_bytes[0];
				data_into_current_block[1] = next_block_pos_bytes[1];
				data_into_current_block[2] = next_block_pos_bytes[2];
				data_into_current_block[3] = next_block_pos_bytes[3];

				direct_block_write(&driveFile,RawDataBlock{
					hash : [0; 32],
					blockPosition : block_index,
					blockTypeId : 0b000100000,
					data : data_into_current_block
				},block_index );
				return block_index
				
			}
		}
		
	}
	
}




//this will read the requested data from a chain of blocks
pub fn data_read(driveFile : &File,start_index : u32, mut size : usize, mut ts_block_size : usize , offset : usize) -> Vec<u8>{
	let mut data : Vec<u8> = Vec::with_capacity(size);
	let current_data_block = data_block_read(driveFile,start_index);
	let block_data = current_data_block.get_data();
	for i in offset..ts_block_size{
		data.push(block_data[i])
	}
	size -= ts_block_size-offset;
	if size < 468{
		ts_block_size = size
	}
	if size == 0 {
		data
	}else {
		data.append(&mut data_read(driveFile,data_block_read(driveFile,start_index).get_next_block_pos(),size,ts_block_size,0));
		data
	}
}


//this will write to a file's data blocks
pub fn data_write(driveFile : &File, mut data : VecDeque<u8>,offset: usize,mut size : usize, file_start_block : u32,current_total_size_of_file : u32, mut ts_block_size : usize, mut amnt_written : u32) -> u32{
	let block_offset = offset/448; 
	let inner_block_offset = offset%448;
	let first_data_block_in_file = start_block_read(driveFile,file_start_block).firstDataBlockPos; 
	//this starts the chain at the correct offset
	let mut block_to_be_changed = data_block_read(&driveFile.try_clone().unwrap(),data_block_read(&driveFile.try_clone().unwrap(),first_data_block_in_file).get_data_block_pos_from_block_offset(driveFile,u32::try_from(block_offset).unwrap()));
	let current_block_pos = block_to_be_changed.blockPosition;
	let current_block_ptr = block_to_be_changed.nextDataBlockPos;
	let mut block_data = block_to_be_changed.data;
	if size < 448{
		ts_block_size = size;
	}
	println!("{}",ts_block_size);

	for i in inner_block_offset..ts_block_size+inner_block_offset{
		block_data[i] = data.pop_front().unwrap();
	}

	block_to_be_changed.data = block_data;
	let data_block_pos = block_to_be_changed.blockPosition;
	data_block_write(driveFile,block_to_be_changed, data_block_pos);
	size -= ts_block_size;
	amnt_written += u32::try_from(ts_block_size).unwrap();
	if current_block_ptr == u32::MAX && size != 0{
		//This means that we need to get more blocks
		println!("comindeiring more blocks YARR");
		let mut stolen_indexes = find_empty_block_indexes(&driveFile.try_clone().unwrap(),size/448+1);
		let first_new_index = stolen_indexes[stolen_indexes.len()-1];
		
		for _j in 0..stolen_indexes.len(){
			
			if stolen_indexes.len() != 1{
				data_block_write(
					driveFile,
					DataBlock::new(
						[0;32],
						0,
						0b00010000,
						[0;448],
						stolen_indexes[stolen_indexes.len()-2]					
					),
					stolen_indexes.pop().unwrap()
				);
			} else {
				data_block_write(
					driveFile,
					DataBlock::new(
						[0;32],
						0,
						0b00010000,
						[0;448],
						u32::MAX					
					),
					stolen_indexes.pop().unwrap()
				);
				break;
			}
		}
		//this will change the current block to point to the first of the new blocks
		data_block_next_pos_modify(driveFile,current_block_pos,first_new_index);
	}
	if size >= 448{
		ts_block_size = 448;
	}else {
		ts_block_size = size;
	}

	if size == 0 {
		amnt_written
	}else{
		data_write(driveFile,data,offset+468,size, first_data_block_in_file,current_total_size_of_file,ts_block_size,amnt_written)
		
	}
	
	
	
	
}
// pub struct DataBlock {
// 	pub hash : [u8; 32],
// 	blockPosition : u32,
// 	blockTypeId : u32,
// 	data : [u8; 468],
// 	nextDataBlockPos : u32,
// }

// This will get you a vector with the requested number of indexes to empty blocks
pub fn find_empty_block_indexes(driveFile : &File,num_of_blocks_needed : usize) -> Vec<u32>{
	let mut i : u32 = 0;
	let mut block_index_vec : Vec<u32> = Vec::with_capacity(256);

	loop {
		
		let checkBlock = direct_block_read(&driveFile, i);
		if RawDataBlock::from(checkBlock).hash == [0;32]{
			//This means that this block is empty
			block_index_vec.push(i);
			if block_index_vec.len() == num_of_blocks_needed{
				break;
			}		
		}
		i += 1;
	}
	block_index_vec
}




pub fn create_dir(driveFile : &File,mut name : OsString, mode : u32, parent_start_index : u32, uid : u32, gid : u32) -> MetaData{
	let indexes = find_empty_block_indexes(driveFile,2);
	let start_block_pos = indexes[0];
	let data_block_pos = indexes[1];
	let temp_arr : [u8;128] = [0;128];
	
	name.push(OsString::from(String::from_utf8(temp_arr.to_vec()).unwrap()));
	let mut temp_name_str_buff = String::from(name.to_str().expect("name could not become string"));
	temp_name_str_buff.truncate(128);
	let parsed_name = OsString::from(temp_name_str_buff);
	let meta_data = 
	MetaData{
		size: 468,
		blockLen : 1,
		aTime : 0,
		mTime : 0,
		cTime: 0,
		perm : u16::try_from(mode).unwrap(),
		uid,
		gid,
		fileType : 8
	};
	let parsed_name = <[u8; 128]>::try_from(parsed_name.to_str().expect("name can't be string").as_bytes()).expect("name did not become array");
	let new_start_block = StartBlock::new(
		[0;32], 								// hash
		start_block_pos,						// pos
		404,								// Type id (directory)
		parsed_name,
		meta_data.clone(),		
		data_block_pos,
		[0;32],
		data_block_pos,
		[0;32]
	);

	start_block_write(
		driveFile,
		new_start_block,
		start_block_pos
	);

	println!("{}",data_block_pos);
	let new_data_block = DataBlock::new(
		[0;32], //Hash
		data_block_pos, //pos
		42, //type (data)
		[0;448],
		u32::MAX
	);
	println!("{}",start_block_pos);
	data_block_write(
		driveFile,new_data_block,data_block_pos
	);


	add_directory_entry(driveFile,&start_block_read(&driveFile,parent_start_index),start_block_pos);
	meta_data
}

pub fn create_file(driveFile : &File,mut name : OsString,mode : u16, parent_start_index : u32, uid : u32, gid : u32) -> MetaData{
	let indexes = find_empty_block_indexes(driveFile,2);
	let start_block_pos = indexes[0];
	let data_block_pos = indexes[1];
	let temp_arr : [u8;128] = [0;128];
	
	name.push(OsString::from(String::from_utf8(temp_arr.to_vec()).unwrap()));
	let mut temp_name_str_buff = String::from(name.to_str().expect("name could not become string"));
	temp_name_str_buff.truncate(128);
	let parsed_name = OsString::from(temp_name_str_buff);


	let meta_data = 
	MetaData{
		size: 468,
		blockLen : 1,
		aTime : 0,
		mTime : 0,
		cTime: 0,
		perm : u16::try_from(mode).unwrap(),
		uid,
		gid,
		fileType : 16
	};
	let parsed_name = <[u8; 128]>::try_from(parsed_name.to_str().expect("name can't be string").as_bytes()).expect("name did not become array");
	let new_start_block = StartBlock::new(
		[0;32], 								// hash
		start_block_pos,						// pos
		42,								// Type id (directory)
		parsed_name,
		meta_data.clone(),		
		data_block_pos,
		[0;32],
		data_block_pos,
		[0;32]
	);

	start_block_write(
		driveFile,
		new_start_block,
		start_block_pos
	);

	println!("{}",data_block_pos);
	let new_data_block = DataBlock::new(
		[0;32], //Hash
		data_block_pos, //pos
		42, //type (data)
		[0;448],
		u32::MAX
	);
	println!("{}",start_block_pos);
	data_block_write(
		driveFile,new_data_block,data_block_pos
	);


	add_directory_entry(driveFile,&start_block_read(&driveFile,parent_start_index),start_block_pos);
	meta_data
	
}



pub fn add_directory_entry(driveFile : &File, parent_start_block : &StartBlock, entry_index : u32){
	let first_data_block = get_data_block_from_start_block(driveFile,parent_start_block);
	let data_block_to_be_changed = data_block_read(driveFile,first_data_block.get_data_block_pos_from_block_offset(driveFile,entry_index/117));		
	modify_directory_entry_block_from_slot_and_block(driveFile,data_block_to_be_changed,get_first_empty_directory_entry_slot(driveFile,&get_data_block_from_start_block(driveFile,parent_start_block)),entry_index);
}


pub fn get_first_empty_directory_entry_slot(driveFile : &File, data_block : &DataBlock) -> u32{
	let mut data_idr = data_block.clone().get_data().into_iter();
	let mut perhapsNumber : u32;
	let	mut buffer_for_bytes : [u8; 4] = [0;4];
	for i in 0..112{
		buffer_for_bytes[0] = data_idr.next().unwrap();
		buffer_for_bytes[1] = data_idr.next().unwrap();
		buffer_for_bytes[2] = data_idr.next().unwrap();
		buffer_for_bytes[3] = data_idr.next().unwrap();
		perhapsNumber = u32::from_le_bytes(buffer_for_bytes);
		if perhapsNumber == 0{
			
			return u32::try_from(i).unwrap();
			
		}
	}
	get_first_empty_directory_entry_slot(&driveFile,&get_next_data_block(&driveFile,data_block.clone()))
	
}

pub fn modify_directory_entry_block_from_slot_and_block(driveFile : &File, mut data_block : DataBlock, slot : u32, entry_index : u32){
	let mut data = data_block.clone().clone().get_data();
	let mut byte_offset : usize = usize::try_from(slot*4).unwrap();
	for byte in entry_index.to_le_bytes(){
		data[byte_offset] = byte;
		byte_offset += 1;
	}
	data_block.set_data(data);
	data_block_write(driveFile,data_block.clone(),data_block.blockPosition);
}



pub fn get_data_block_from_start_block(driveFile : &File, start_block: &StartBlock) -> DataBlock{
	data_block_read(&driveFile,start_block.get_data_start_pos())
}

pub fn get_next_data_block(driveFile : &File, current_block : DataBlock) -> DataBlock{
	data_block_read(driveFile,current_block.nextDataBlockPos)
}




pub fn read_start_block_name(driveFile: &File, block_index : u32) -> OsString{
	let block = start_block_read(&driveFile,block_index);
	block.get_name()
}


pub fn delete_directory_entry(driveFile : &File, directory_start_block_index : u32, ptr_to_be_removed: u32){
	let directory_data_block = data_block_read(driveFile,start_block_read(driveFile,directory_start_block_index).firstDataBlockPos);
	
	let directory_slot = directory_data_block.find_directory_entry_slot(ptr_to_be_removed);
	modify_directory_entry_block_from_slot_and_block(driveFile,directory_data_block,directory_slot,0)



}


pub fn delete_file(driveFile: &File, start_block_index : u32, directory_entry_block : u32){
	// first I need to remove the directory entry
	delete_directory_entry(driveFile,directory_entry_block,start_block_index);
	
	// I need to get all of the idexes of the blocks that need to be taken out back and kindly shoved into a volcano
	let mut block_indexes : Vec<u32> = Vec::with_capacity(256);
	block_indexes.push(start_block_index);
	block_indexes.push(data_block_read(driveFile,start_block_read(driveFile,start_block_index).firstDataBlockPos).blockPosition);
	let mut current_index = block_indexes[1];
	
	loop {
		let data_block = data_block_read(driveFile,current_index);
		block_indexes.push(data_block.blockPosition);
		current_index = data_block.nextDataBlockPos;

		if current_index == u32::MAX{
			break;	
		}
	}
	// now that we have a hit list we must finish the job
	for index in block_indexes{
		direct_block_delete(driveFile,index);
	}
	// MURDER :yayayayayay:
	
	

}




