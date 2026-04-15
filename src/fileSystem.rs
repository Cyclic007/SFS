use fuse_mt::*;
struct SecureFileSystem{}
use std::io;
impl FilesystemMT for SecureFileSystem{

	fn init(&self, _req: RequestInfo) -> ResultEmpty {
		Ok(()) 
	}


    fn destroy(&self) { }

    
    fn getattr(
        &self,
        _req: RequestInfo,
        _path: &Path,
        _fh: Option<u64>,
    ) -> ResultEntry {
		
	
    }

    
    fn chmod(
        &self,
        _req: RequestInfo,
        _path: &Path,
        _fh: Option<u64>,
        _mode: u32,
    ) -> ResultEmpty { ... }
    fn chown(
        &self,
        _req: RequestInfo,
        _path: &Path,
        _fh: Option<u64>,
        _uid: Option<u32>,
        _gid: Option<u32>,
    ) -> ResultEmpty { ... }
    fn truncate(
        &self,
        _req: RequestInfo,
        _path: &Path,
        _fh: Option<u64>,
        _size: u64,
    ) -> ResultEmpty { ... }
    fn utimens(
        &self,
        _req: RequestInfo,
        _path: &Path,
        _fh: Option<u64>,
        _atime: Option<SystemTime>,
        _mtime: Option<SystemTime>,
    ) -> ResultEmpty { ... }
    fn utimens_macos(
        &self,
        _req: RequestInfo,
        _path: &Path,
        _fh: Option<u64>,
        _crtime: Option<SystemTime>,
        _chgtime: Option<SystemTime>,
        _bkuptime: Option<SystemTime>,
        _flags: Option<u32>,
    ) -> ResultEmpty { ... }
    fn readlink(&self, _req: RequestInfo, _path: &Path) -> ResultData { ... }
    fn mknod(
        &self,
        _req: RequestInfo,
        _parent: &Path,
        _name: &OsStr,
        _mode: u32,
        _rdev: u32,
    ) -> ResultEntry { ... }
    fn mkdir(
        &self,
        _req: RequestInfo,
        _parent: &Path,
        _name: &OsStr,
        _mode: u32,
    ) -> ResultEntry { ... }
    fn unlink(
        &self,
        _req: RequestInfo,
        _parent: &Path,
        _name: &OsStr,
    ) -> ResultEmpty { ... }
    fn rmdir(
        &self,
        _req: RequestInfo,
        _parent: &Path,
        _name: &OsStr,
    ) -> ResultEmpty { ... }
    fn symlink(
        &self,
        _req: RequestInfo,
        _parent: &Path,
        _name: &OsStr,
        _target: &Path,
    ) -> ResultEntry { ... }
    fn rename(
        &self,
        _req: RequestInfo,
        _parent: &Path,
        _name: &OsStr,
        _newparent: &Path,
        _newname: &OsStr,
    ) -> ResultEmpty { ... }
    fn link(
        &self,
        _req: RequestInfo,
        _path: &Path,
        _newparent: &Path,
        _newname: &OsStr,
    ) -> ResultEntry { ... }
    fn open(&self, _req: RequestInfo, _path: &Path, _flags: u32) -> ResultOpen { ... }
    fn read(
        &self,
        _req: RequestInfo,
        _path: &Path,
        _fh: u64,
        _offset: u64,
        _size: u32,
        callback: impl FnOnce(ResultSlice<'_>) -> CallbackResult,
    ) -> CallbackResult { ... }
    fn write(
        &self,
        _req: RequestInfo,
        _path: &Path,
        _fh: u64,
        _offset: u64,
        _data: Vec<u8>,
        _flags: u32,
    ) -> ResultWrite { ... }
    fn flush(
        &self,
        _req: RequestInfo,
        _path: &Path,
        _fh: u64,
        _lock_owner: u64,
    ) -> ResultEmpty { ... }
    fn release(
        &self,
        _req: RequestInfo,
        _path: &Path,
        _fh: u64,
        _flags: u32,
        _lock_owner: u64,
        _flush: bool,
    ) -> ResultEmpty { ... }
    fn fsync(
        &self,
        _req: RequestInfo,
        _path: &Path,
        _fh: u64,
        _datasync: bool,
    ) -> ResultEmpty { ... }
    fn opendir(
        &self,
        _req: RequestInfo,
        _path: &Path,
        _flags: u32,
    ) -> ResultOpen { ... }
    fn readdir(
        &self,
        _req: RequestInfo,
        _path: &Path,
        _fh: u64,
    ) -> ResultReaddir { ... }
    fn releasedir(
        &self,
        _req: RequestInfo,
        _path: &Path,
        _fh: u64,
        _flags: u32,
    ) -> ResultEmpty { ... }
    fn fsyncdir(
        &self,
        _req: RequestInfo,
        _path: &Path,
        _fh: u64,
        _datasync: bool,
    ) -> ResultEmpty { ... }
    fn statfs(&self, _req: RequestInfo, _path: &Path) -> ResultStatfs { ... }
    fn setxattr(
        &self,
        _req: RequestInfo,
        _path: &Path,
        _name: &OsStr,
        _value: &[u8],
        _flags: u32,
        _position: u32,
    ) -> ResultEmpty { ... }
    fn getxattr(
        &self,
        _req: RequestInfo,
        _path: &Path,
        _name: &OsStr,
        _size: u32,
    ) -> ResultXattr { ... }
    fn listxattr(
        &self,
        _req: RequestInfo,
        _path: &Path,
        _size: u32,
    ) -> ResultXattr { ... }
    fn removexattr(
        &self,
        _req: RequestInfo,
        _path: &Path,
        _name: &OsStr,
    ) -> ResultEmpty { ... }
    fn access(&self, _req: RequestInfo, _path: &Path, _mask: u32) -> ResultEmpty { ... }
    fn create(
        &self,
        _req: RequestInfo,
        _parent: &Path,
        _name: &OsStr,
        _mode: u32,
        _flags: u32,
    ) -> ResultCreate {
    }
}	

