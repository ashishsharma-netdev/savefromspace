use serde::Deserialize;
use std::{process::{Command, Stdio}, path::PathBuf};
#[derive(Deserialize)] struct DownloadRequest { id:String, url:String, kind:String, format:String, playlist:bool, folder:String }
#[tauri::command] fn start_download(request:DownloadRequest)->Result<String,String>{
 if request.url.trim().is_empty(){return Err("URL is required".into());}
 let mut args:Vec<String>=Vec::new(); if !request.playlist {args.push("--no-playlist".into());}
 if !request.folder.trim().is_empty(){args.extend(["-P".into(),request.folder.clone()]);}
 if request.kind=="audio" || (request.kind=="video" && ["mp3","aac","m4a","wav","flac","opus"].contains(&request.format.as_str())){
  args.extend(["-x".into(),"--audio-format".into(),request.format.clone()]);
 }else if request.kind=="video" {args.extend(["--format".into(),"bestvideo+bestaudio/best".into(),"--merge-output-format".into(),request.format.clone()]);}
 args.push(request.url.clone());
 let child=Command::new("yt-dlp").args(args).stdout(Stdio::null()).stderr(Stdio::null()).spawn().map_err(|e|format!("Could not start yt-dlp. Install yt-dlp and add it to PATH. {e}"))?;
 let _=child; Ok(format!("Download job {} launched. Add job monitoring in the next iteration.",request.id))
}
#[cfg_attr(mobile, tauri::mobile_entry_point)] pub fn run(){tauri::Builder::default().invoke_handler(tauri::generate_handler![start_download]).run(tauri::generate_context!()).expect("error while running SaveFromSpace");}
