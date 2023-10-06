#[derive(Default, Debug)]
struct Comment<'me> {
    contents: Vec<&'me str>,
}

impl<'me> Comment<'me> {
    fn of(source: &'me str) -> Vec<Comment<'me>> {
        let mut comments = Vec::new();
        let mut comment = Comment::default();

        for line in source.lines().map(str::trim_start) {
            match line.strip_prefix("//") {
                Some(contents) => {
                    let contents = contents.strip_prefix(' ').unwrap_or(contents);
                    comment.contents.push(contents);
                }
                None => {
                    if !comment.is_empty() {
                        let comment = std::mem::take(&mut comment);
                        comments.push(comment);
                    }
                }
            }
        }

        if !comment.is_empty() {
            comments.push(comment);
        }

        comments
    }

    fn is_empty(&self) -> bool {
        self.contents.is_empty()
    }
}

impl<'me, I: std::slice::SliceIndex<[&'me str]>> std::ops::Index<I> for Comment<'me> {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.contents[index]
    }
}

pub fn for_each(mut f: impl FnMut(&str, &str)) {
    let current_dir = std::env::current_dir().unwrap().join("src");

    traverse(&current_dir, &mut |path| {
        let source = std::fs::read_to_string(path)
            .unwrap_or_else(|kind| panic!("reading `{}`: {kind}", path.display()));

        for comment in Comment::of(&source) {
            let Some(name) = &comment[0].strip_prefix("test ") else {
                continue;
            };
            let text = comment[1..].to_vec().join("\n");

            f(name, &text)
        }
    })
    .unwrap();
}

fn traverse(dir: &std::path::Path, cb: &mut dyn FnMut(&std::path::Path)) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let path = entry?.path();
            if path.is_dir() {
                traverse(&path, cb)?;
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                cb(&path);
            }
        }
    }
    Ok(())
}
