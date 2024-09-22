/*
Mandy by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "coutils"
/// crate to perform various
/// file-related operations.
use coutils;

/// Importing the "PathBuf"
/// structure from Rust's standard
/// library.
use std::path::PathBuf;

/// Importing Mandy's structure
/// to catch and handle errors.
use super::err::MandyErr;

/// Importing the structure
/// to build sitemaps.
use super::units::SiteMap;

/// Importing the structure
/// to build URLs for sitemaps.
use super::units::SiteMapUrl;

/// Importing Rust's standard
/// API for working with maps.
use std::collections::HashMap;

/// Importing the "MandyContent"
/// structure to read content files
/// written in the Markdown format.
use super::units::MandyContent;

/// Importing the function
/// to read a Mandy project's 
/// configuration settings.
use super::gather::read_config;

/// Attempts to generate a sitemap and a "robots.txt" file. If this fails, an error is returned.
pub fn seo(dir: &String, content_files: &HashMap<PathBuf,MandyContent>) -> Result<(),MandyErr>{
    let config = match read_config(dir){
        Ok(config) => config,
        Err(e) => return Err::<(), MandyErr>(MandyErr::new(&e.to_string()))
    };
    if config.contents.seo{
        let mut sitemap_buf: PathBuf = PathBuf::new();
        let mut robots_buf: PathBuf = PathBuf::new();
        let mut dist_buf: PathBuf = PathBuf::new();
        sitemap_buf.push(dir);
        sitemap_buf.push(&config.contents.dist_dir);
        sitemap_buf.push("sitemap.xml");
        robots_buf.push(dir);
        robots_buf.push(&config.contents.dist_dir);
        robots_buf.push("robots.txt");
        dist_buf.push(dir);
        dist_buf.push(&config.contents.dist_dir);
        let mut map_urls: Vec<SiteMapUrl> = Vec::new();
        for (_key,value) in content_files.into_iter().enumerate(){
            let (_path,content) = value;
            let full_url: String = format!(
                "{}{}", 
                config.contents.tl_domain, 
                content.url
            );
            map_urls.push(SiteMapUrl{ url: full_url });
        }
        let sitemap: SiteMap = SiteMap{ urls: map_urls };
        let robots_txt: String = format!("User-Agent: *\nDisallow:\n\nSitemap: {}/sitemap.xml", &config.contents.tl_domain);
        if dist_buf.exists(){
            if !(sitemap_buf.exists()) && !(robots_buf.exists()){
                let _sm_create_op: () = match coutils::create_file(&sitemap_buf.display().to_string()){
                    Ok(_sm_create_op) => _sm_create_op,
                    Err(e) => return Err::<(), MandyErr>(MandyErr::new(&e.to_string()))
                };
                let _sm_write_op: () = match coutils::write_to_file(&sitemap_buf.display().to_string(), &sitemap.to_string()){
                    Ok(_sm_write_op) => _sm_write_op,
                    Err(e) => return Err::<(), MandyErr>(MandyErr::new(&e.to_string()))
                };
                let _robots_create_op: () = match coutils::create_file(&robots_buf.display().to_string()){
                    Ok(_robots_create_op) => _robots_create_op,
                    Err(e) => return Err::<(), MandyErr>(MandyErr::new(&e.to_string()))
                };
                let robots_write_op: () = match coutils::write_to_file(&robots_buf.display().to_string(), &robots_txt){
                    Ok(robots_write_op) => robots_write_op,
                    Err(e) => return Err::<(), MandyErr>(MandyErr::new(&e.to_string()))
                };
                Ok(robots_write_op)
            }
            else {
                let e: String = format!("The files \"{}\" and \"{}\" already exist.", &sitemap_buf.display().to_string(), &robots_buf.display().to_string());
                Err::<(), MandyErr>(MandyErr::new(&e.to_string()))
            }
        }
        else {
            let e: String = format!("The directory \"{}\" does not exist.", &dist_buf.display().to_string());
            Err::<(), MandyErr>(MandyErr::new(&e.to_string()))
        }
    }
    else {
        Ok(())
    }
    
}