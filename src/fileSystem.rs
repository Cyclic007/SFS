use fuse_mt::*;
struct SecureFileSystem{}
use std::io;
use std::path::Path;
use super::handle;
const HANDLE_TIME_TO_LIVE: Duration = Duration::from_secs(365*24*60*60);
impl FilesystemMT for SecureFileSystem{

	fn init(&self, _req: RequestInfo) -> ResultEmpty {
		Ok(()) 
	}


    fn destroy(&self) { }

    
    fn getattr(
        &self,
        _req: RequestInfo,
        path: &Path,
        fh: Option<u64>,
    ) -> ResultEntry {
			if let Some(handle) = fh{
				debug!("handle exists");
				return ok(
					(
						HANDLE_TIME_TO_LIVE,
						handle::read_handle(handle)
					)
				)
			}
	
    }

}	

