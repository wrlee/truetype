// Parse and output information about a TrueType font file's metadata using the `fonttools` crate which provides tools for parsing and analyzing font files. This uses `fonttools` to read metadata from a TrueType or OpenType font file outputing to the console.

use std::env;
use std::fs::File;
use fonttools::font;
use fonttools::font::Table;

fn err_out(err: &str) -> ! {
  eprintln!("{}", &err.to_string());
  std::process::exit(1);
}
// fn file_err(err: std::io::Error) -> File {
//   err_out(&err.to_string());
// }
// fn font_err(err: Box<dyn std::error::Error>) -> font::Font{
//   err_out(&err.to_string());
// }

fn main() {
    // Open the TrueType font file and read its metadata
    // let font = Font::from_file("path/to/font.ttf").unwrap();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!(r#"
Output meta data for TrueType (TTF) and OpenType (OTF) files.

   Usage: {} <filename>");
        "#, args[0]);
        return;
    }

    let filename = &args[1];
    let file = File::open(filename).unwrap_or_else(|err| err_out(&err.to_string()));

    let mut fnt = font::load(file).unwrap_or_else(|err| err_out(&err.to_string()));

    let _tables = [
      b"acnt", b"ankr", b"avar", b"bdat", b"bhed", b"bloc", b"bsln", b"cmap", b"cvar", b"cvt ", b"EBSC", b"fdsc", b"feat", b"fmtx", b"fond", b"fpgm", b"fvar", b"gasp", b"gcid", b"glyf", b"gvar", b"hdmx",
      b"head",
      b"hhea", b"hmtx", b"just", b"kern", b"kerx", b"lcar", b"loca", b"ltag", b"maxp", b"meta", b"mort", b"morx",
      b"name",
      b"opbd", b"OS/2", b"post", b"prep", b"prop", b"sbix", b"trak", b"vhea", b"vmtx", b"xref", b"Zapf"
    ];
    // for t in _tables { println!("{}", String::from_utf8_lossy(t)); }

    if let Table::Name(name_table) = fnt.get_table(b"name")
      .expect("Error reading name table")
      .expect("There was no name table")
    {
      for rec in &name_table.records {
        // println!("{:?}", rec);

        let platform = match &rec.platformID {
          0 => "Unicode  ",
          1 => "Macintosh",
          2 => "unused   ",
          3 => "Microsoft",
          _ => "_unknown_"
        };
        let title = match &rec.nameID {
          0 => Some("Copyright"),
          1 => Some("Font family"),
          2 => Some("Sub-family"),
          3 => Some("Sub-family ID"),
          4 => Some("Full name"),
          5 => Some("Version"),
          6 => Some("PostScript name"),
          7 => Some("Trademark"),
          8 => Some("Manufacture"),
          9 => Some("Designer"),
          10 => Some("Description"),
          11 => Some("Vendor URL"),
          12 => Some("Designer URL"),
          13 => Some("License"),
          14 => Some("License URL"),
          15 => Some("reserved"),
          16 => Some("Preferred family"),
          17 => Some("Preferred sub-family"),
          18 => Some("Compatible fullname"),
          19 => Some("Sample text"),
          20 ..= 24 => Some("_open type_"),
          25 => Some("Variations PostScript prefix"),
          26 ..= 255 => Some("_reserved for the future_"),
          _ => None,
        };
        match title {
          None => println!("{} name[{}]\t\"{}\"", platform, rec.nameID, rec.string),
          Some(title) => println!("{} name[{}]\t{}:\t\"{}\"", platform, rec.nameID, title, rec.string),
        }
      }
    }

    // if let Table::Head(head_table) = fnt.get_table(b"head")
    //   .expect("Error reading head table")
    //   .expect("There was no head table") {
    let table_option = fnt.get_table(b"head").unwrap();
    if table_option != None {
      if let Table::Head(head_table) = table_option.unwrap() {
        // println!("head: Version {}.{}", head_table.majorVersion, head_table.minorVersion);
        println!("head: Revision {}", head_table.fontRevision);
        println!("head: Created {}", head_table.created);
        println!("head: Modified {}", head_table.modified);
      }
    }

    // if let Table::Hhea(hhea_table) = fnt.get_table(b"hhea")
    //   .expect("Error reading hhea table")
    //   .expect("There was no hhea table") {
    //     println!("{:#?}", hhea_table);
    // }
    // if let Table::Cmap(cmap_table) = fnt.get_table(b"cmap")
    //   .expect("Error reading cmap table")
    //   .expect("There was no cmap table") {
    //     println!("{:?}", cmap_table);
    // }
    // if let Table::Hmtx(hmtx_table) = fnt.get_table(b"hmtx")
    //   .expect("Error reading hmtx table")
    //   .expect("There was no hmtx table") {
    //     println!("{:?}", hmtx_table);
    // }
    // if let Table::Maxp(maxp_table) = fnt.get_table(b"maxp")
    //   .expect("Error reading maxp table")
    //   .expect("There was no maxp table") {
    //     println!("{:?}", maxp_table);
    // }
    // if let Table::Post(post_table) = fnt.get_table(b"post")
    //   .expect("Error reading post table")
    //   .expect("There was no post table") {
    //     println!("{:#?}", post_table);
    // }

    // Print the font metadata to the console
    // println!("Font name: {}", fnt.name.as_ref().unwrap().font_family());
    // println!("Font version: {}", fnt.version.unwrap_or("Unknown version"));
    // println!("Number of glyphs: {}", fnt.glyph_count());
    // println!("Number of tables: {}", fnt.tables.len());

    // Print information about each table
    // for table in fnt.tables {
        // println!("{:?}", table);
        // println!("Table: {}", table.tag());
        // println!("Offset: {}", table.offset);
        // println!("Length: {}", table.length);
    // }
}