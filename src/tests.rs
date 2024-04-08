#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn path_name() {
        let dir = std::env::current_dir().unwrap();
        let name = &dir.to_str().unwrap().split("\\").last().unwrap();

        assert_eq!(name, &"clout");
    }

    #[test]
    fn compiler_files() {
        let files: Vec<_> = fs::read_dir("./test")
            .unwrap()
            .map(|res| res.map(|e| e.path()))
            .filter(|x| x.is_ok())
            .map(|x| x.unwrap())
            .filter(|x| x.to_string_lossy().contains(".cpp"))
            .collect();

        assert_eq!(files[0].file_name().unwrap().to_string_lossy(), "main.cpp")
    }
}
