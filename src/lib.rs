use rand::seq::SliceRandom;
use rand::Rng;

const BROWSERS: &[&str] = &[
    "Chrome", "Firefox", "Safari", "Edge", "Opera", 
    "Brave", "Vivaldi", "Internet Explorer", "Samsung Internet", 
    "UC Browser", "Tor Browser", "Maxthon", "SeaMonkey", 
    "Pale Moon", "Yandex Browser", "Epic", "Avant Browser", 
    "Lynx", "Midori", "Konqueror", "Netscape", 
    "Camino", "Flock", "RockMelt", "Comodo Dragon", 
    "Sleipnir", "SRWare Iron", "360 Secure Browser", "Waterfox", 
    "Qutebrowser", "SlimBrowser", "Tor Browser", "Falkon", 
    "Dooble", "QtWeb", "Otter Browser", "Lunascape", 
    "Cốc Cốc", "BlackHawk", "Superbird", "Cent Browser", 
    "Ghost Browser", "Basilisk", "Min Browser", "Light", 
    "Avast Secure Browser", "Polarity", "Coowon Browser", 
    "Colibri", "Torch", "Cyberfox", "Slimjet"
];

const OS: &[&str] = &[
    "Windows NT 10.0", "Windows NT 6.1", "Windows NT 6.2", 
    "Windows NT 5.1", "Windows NT 5.0", "Mac OS X 10_15_7", 
    "Mac OS X 10_14_6", "Mac OS X 10_13_6", "Linux x86_64", 
    "Linux i686", "Linux armv7l", "Android 11", 
    "Android 10", "Android 9", "Android 8.1.0", 
    "Android 7.0", "iPhone OS 14_6", "iPhone OS 13_5", 
    "iPhone OS 12_4", "iPad; CPU OS 13_2 like Mac OS X", 
    "iPad; CPU OS 14_4 like Mac OS X", "FreeBSD", 
    "NetBSD", "OpenBSD", "Solaris 11", 
    "Ubuntu 20.04", "Ubuntu 18.04", "Debian 10", 
    "Debian 9", "Fedora 34", "Fedora 33", 
    "CentOS 7", "CentOS 8", "Arch Linux", 
    "Manjaro", "Kali Linux", "Raspbian", 
    "Chrome OS", "Haiku", "ReactOS", 
    "Zorin OS", "Elementary OS", "Pop!_OS", 
    "Gentoo Linux", "Mageia", "Slackware", 
    "Puppy Linux", "Solus", "Deepin", 
    "openSUSE Leap", "openSUSE Tumbleweed"
];

const DEVICES: &[&str] = &[
    "Win64; x64", "Win32; x86", "Intel Mac OS X", 
    "X11; Linux x86_64", "X11; Linux i686", 
    "Mobile; rv:89.0", "CPU iPhone OS", "iPad; CPU OS", 
    "Samsung SM-G950U", "Samsung SM-N960U", 
    "Huawei P30 Pro", "OnePlus A6013", "Google Pixel 5", 
    "Xiaomi Mi 9", "Sony Xperia XZ3", 
    "LG G8 ThinQ", "HTC U11", "Nokia 7.2", 
    "Motorola G7 Plus", "Lenovo Tab M10", 
    "Asus ROG Phone", "Razer Phone 2", "ZTE Axon 10 Pro", 
    "Oppo Find X2", "Vivo NEX 3", "Realme X50 Pro", 
    "Honor 20 Pro", "BlackBerry KEY2", "Palm Phone", 
    "Microsoft Surface Duo", "Amazon Kindle Fire", 
    "Nintendo Switch", "PlayStation 4", "Xbox One", 
    "Raspberry Pi 4", "BeagleBone Black", "Arduino Uno", 
    "Dell XPS 13", "HP Spectre x360", "Lenovo ThinkPad X1", 
    "Asus ZenBook 14", "Acer Swift 3", "MSI GS66 Stealth", 
    "Alienware M15", "Apple MacBook Pro", "Apple MacBook Air", 
    "Google Chromebook", "Huawei MateBook X Pro", "Samsung Galaxy Book", 
    "Sony VAIO", "Toshiba Portege"
];

const WEBKITS: &[&str] = &[
    "537.36", "605.1.15", "534.30", "601.1.56", "533.1", 
    "537.71", "538.1", "534.59.10", "537.73.11", 
    "602.1.50", "604.4.7", "606.4.5", "607.3.1", 
    "608.2.11", "609.1.22", "610.4.1", "611.2.7", 
    "612.1.7", "613.4.5", "614.3.1", "615.1.8", 
    "616.4.5", "617.1.8", "618.2.11", "619.1.22", 
    "620.4.1", "621.2.7", "622.1.7", "623.4.5", 
    "624.3.1", "625.1.8", "626.4.5", "627.1.8", 
    "628.2.11", "629.1.22", "630.4.1", "631.2.7", 
    "632.1.7", "633.4.5", "634.3.1", "635.1.8", 
    "636.4.5", "637.1.8", "638.2.11", "639.1.22", 
    "640.4.1", "641.2.7", "642.1.7", "643.4.5", 
    "644.3.1", "645.1.8"
];

const BASES: &[&str] = &[
    "Mozilla/5.0", "Mozilla/4.0", "Opera/9.80", "Mozilla/3.0", 
    "Mozilla/2.0", "Mozilla/1.0", "Opera/10.00", "Opera/12.16",
    "Mozilla/6.0", "Mozilla/7.0", "Opera/11.62", "Mozilla/8.0", 
    "Mozilla/9.0", "Opera/8.54", "Opera/7.54", "Opera/6.03", 
    "Opera/5.12", "Opera/4.02", "Mozilla/0.9", "Mozilla/0.8", 
    "Mozilla/10.0", "Mozilla/11.0", "Mozilla/12.0", "Opera/9.26", 
    "Opera/9.64", "Opera/9.23", "Opera/9.52", "Opera/9.27", 
    "Opera/9.60", "Opera/9.02", "Opera/8.65", "Opera/8.51", 
    "Opera/8.50", "Opera/7.60", "Opera/7.50", "Opera/7.20", 
    "Opera/7.11", "Opera/7.03", "Opera/7.01", "Opera/7.0", 
    "Opera/6.12", "Opera/6.11", "Opera/6.10", "Opera/5.14", 
    "Opera/5.13", "Opera/5.11", "Opera/5.10", "Opera/5.02", 
    "Opera/5.01", "Opera/5.0"
];

pub fn user_agent() -> String {
    let mut rng = rand::thread_rng();
    
    let base = BASES.choose(&mut rng).unwrap();
    let browser = BROWSERS.choose(&mut rng).unwrap();
    let os = OS.choose(&mut rng).unwrap();
    let device = DEVICES.choose(&mut rng).unwrap();
    let webkit = WEBKITS.choose(&mut rng).unwrap();
    
    let browser_version: u8 = rng.gen_range(60..100);
    let build_version: u16 = rng.gen_range(0..500);

    format!(
        "{base} ({os}; {device}) AppleWebKit/{webkit} (KHTML, like Gecko) {browser}/{browser_version}.0.{build_version} Safari/{webkit}",
        base = base,
        os = os,
        device = device,
        webkit = webkit,
        browser = browser,
        browser_version = browser_version,
        build_version = build_version
    )
}
   