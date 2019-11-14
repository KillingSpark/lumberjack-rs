#[cfg(test)]
#[test]
fn it_works() {
    use crate::Conf;
    use std::io::Write;

    let path = "./test_logs";

    if std::fs::metadata(&path).is_ok() {
        std::fs::remove_dir_all(&path).unwrap();
    }
    std::fs::create_dir(&path).unwrap();

    let max_files = 4;
    let max_age = 1;
    let conf = Conf {
        max_size: 15,
        max_age: Some(max_age),
        max_files: Some(max_files),

        log_dir: path.into(),
        name_template: "mylog.log".to_owned(),
    };

    let mut logger = crate::new(conf).unwrap();
    logger.write_all("111111111\n".as_bytes()).unwrap();
    logger.write_all("222222222\n".as_bytes()).unwrap();
    logger.write_all("333333333\n".as_bytes()).unwrap();
    logger.write_all("444444444\n".as_bytes()).unwrap();
    logger.write_all("555555555\n".as_bytes()).unwrap();
    logger.write_all("666666666\n".as_bytes()).unwrap();
    logger.write_all("777777777\n".as_bytes()).unwrap();
    logger.write_all("888888888\n".as_bytes()).unwrap();
    logger.write_all("999999999\n".as_bytes()).unwrap();
    logger.write_all("000000000\n".as_bytes()).unwrap();

    let mut resulting_files: Vec<_> = std::fs::read_dir(&path)
        .unwrap()
        .map(|x| x.unwrap())
        .collect();
    assert_eq!(max_files, resulting_files.len());
    
    resulting_files.sort_by(|l, r| l.path().cmp(&r.path()));
    let last_file = &resulting_files[max_files-1];
    let last_file_path = last_file.path();
    let content = std::fs::read_to_string(last_file_path).unwrap();
    for x in resulting_files {
        println!("{:?}", x.path());
    }
    assert_eq!(&content, "999999999\n000000000\n");

    std::fs::remove_dir_all(&path).unwrap();
}
