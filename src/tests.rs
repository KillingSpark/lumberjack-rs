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

    let conf = Conf {
        max_size: 15,
        max_age: Some(1),
        max_files: Some(4),
        
        log_dir: path.into(),
        name_template: "mylog.log".to_owned(),
    };

    let mut logger = crate::new(conf).unwrap();
    logger.write_all("123456789\n".as_bytes()).unwrap();
    logger.write_all("123456789\n".as_bytes()).unwrap();
    logger.write_all("123456789\n".as_bytes()).unwrap();
    logger.write_all("123456789\n".as_bytes()).unwrap();
    logger.write_all("123456789\n".as_bytes()).unwrap();
    logger.write_all("123456789\n".as_bytes()).unwrap();
    logger.write_all("123456789\n".as_bytes()).unwrap();
    logger.write_all("123456789\n".as_bytes()).unwrap();
    logger.write_all("123456789\n".as_bytes()).unwrap();

    let resulting_files: Vec<_> = std::fs::read_dir(&path)
        .unwrap()
        .map(|x| x.unwrap())
        .collect();
    assert_eq!(4, resulting_files.len());

    std::fs::remove_dir_all(&path).unwrap();
}
