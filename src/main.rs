/// AI4OSE Lab1:
///
/// 与AI合作进行操作系统内核学习的起点
fn main() {
    // 读取 content.txt 文件内容
    let content = include_str!("content.md");
    print!("{}", parse_markdown(content));
}

/// 简单的 Markdown 解析器
fn parse_markdown(input: &str) -> String {
    let mut output = String::new();

    // 逐行处理 Markdown 内容
    for line in input.lines() {
        let parsed = if line.starts_with('#') {
            parse_header(line)
        } else if line.starts_with("- ") {
            parse_list_item(line)
        } else {
            line.to_string()
        };
        output.push_str(&parsed);
        output.push('\n');
    }

    output
}

/// 解析 Markdown 标题
fn parse_header(line: &str) -> String {
    let level = line.chars().take_while(|&c| c == '#').count();
    // 提取标题内容（去掉 # 和空格）
    let content = line[level..].trim();
    // 使用 ANSI 颜色代码
    format!("\x1b[1;{}m{}\x1b[0m", 30 + level, content)
}

/// 解析 Markdown 列表项
fn parse_list_item(line: &str) -> String {
    let content = line[2..].trim();
    format!("  • {}", content)
}

#[cfg(test)]
mod tests {
    #[test]
    fn AI4OSE_Lab1_2026S() {
        assert_eq!("ai4ose".to_string() + "-lab1" + "-2026s", "ai4ose-lab1-2026s");
    }
}
