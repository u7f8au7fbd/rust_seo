//マクロを指定
#[macro_export]
macro_rules! cmd_color {
    (black) => {
        "\x1b[30m"
    };
    (red) => {
        "\x1b[31m"
    };
    (green) => {
        "\x1b[32m"
    };
    (yellow) => {
        "\x1b[33m"
    };
    (blue) => {
        "\x1b[34m"
    };
    (magenta) => {
        "\x1b[35m"
    };
    (cyan) => {
        "\x1b[36m"
    };
    (white) => {
        "\x1b[37m"
    };
    (black_b) => {
        "\x1b[40m"
    };
    (red_b) => {
        "\x1b[41m"
    };
    (green_b) => {
        "\x1b[42m"
    };
    (yellow_b) => {
        "\x1b[43m"
    };
    (blue_b) => {
        "\x1b[44m"
    };
    (magenta_b) => {
        "\x1b[45m"
    };
    (cyan_b) => {
        "\x1b[46m"
    };
    (white_b) => {
        "\x1b[47m"
    };
    (reset) => {
        "\x1b[0m"
    };
}
