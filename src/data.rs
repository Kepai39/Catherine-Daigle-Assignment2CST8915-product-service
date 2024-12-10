use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "PlayStation 5".to_string(),
            price: 649.99,
            description: "The PlayStation 5 (PS5) delivers next-gen gaming with ultra-fast load times, stunning 4K visuals, and immersive haptic feedback. Powered by a custom SSD and advanced processing capabilities, the PS5 offers an incredible gaming experience, whether you're exploring vast open worlds or diving into action-packed adventures. With a sleek design, ultra-responsive DualSense controller, and a growing library of exclusive titles, the PS5 is the ultimate console for gamers.".to_string(),
            image: "/ps.jpg".to_string()
        },
        Product {
            id: 2,
            name: "Apple iPhone15 ProMax ".to_string(),
            price: 2099.99,
            description: "Elevate your mobile experience with the all-new iPhone 15 Pro Max! Featuring the powerful A17 Pro chip, a stunning 6.7-inch Super Retina XDR display, and a groundbreaking 48MP camera system, this is the ultimate device for power, performance, and photography.".to_string(),
            image: "/AppleiPhone15ProMax.jpg".to_string()
        },
        Product {
            id: 3,
            name: "Apple Watch SE".to_string(),
            price: 359.99,
            description: "Discover the perfect blend of performance, style, and value with the Apple Watch SE! Packed with essential features like fitness tracking, heart rate monitoring, and seamless integration with your iPhone, the Apple Watch SE is your perfect companion for everyday life.".to_string(),
            image: "/AppleWatchSE.jpg".to_string()
        },
        Product {
            id: 4,
            name: "Bella ProTouchscreen AirFryer".to_string(),
            price: 129.99,
            description: "Transform the way you cook with the Bella Pro Touchscreen Air Fryer! Enjoy crispy, delicious meals with little to no oil. Whether you're air frying, baking, or roasting, this easy-to-use appliance is perfect for all your cooking needs.".to_string(),
            image: "/BellaProTouchscreenAirFryer.jpg".to_string()
        },
        Product {
            id: 5,
            name: "Bose Quiet Comfort".to_string(),
            price: 499.99,
            description: "Step into a world of pure sound and silence with the Bose QuietComfort 45 headphones. Engineered for superior noise-cancellation and all-day comfort, these headphones let you enjoy your music, calls, and podcasts like never before.".to_string(),
            image: "/bosequietcomfort.jpg".to_string()
        },
        Product {
            id: 6,
            name: "Final Fantasy Rebirth".to_string(),
            price: 89.99,
            description: "The next chapter in the legendary Final Fantasy VII saga is here! Final Fantasy VII Rebirth delivers stunning graphics, an immersive story, and intense action-packed gameplay that fans have been waiting for.".to_string(),
            image: "/finalfantasyrebirth.jpg".to_string()
        },
        Product {
            id: 7,
            name: "GoPro HERO13 Black Waterproof".to_string(),
            price: 649.99,
            description: "Unleash the power of the GoPro HERO13 Black, your ultimate companion for every adventure! Whether you're hiking, surfing, or exploring the great outdoors, the HERO13 is built to withstand it all with its rugged, waterproof design.".to_string(),
            image: "/GoProHERO13BlackWaterproof.jpg".to_string()
        },
        Product {
            id: 8,
            name: "JBL Flip5 Waterproof Bluetooth Wireless Speaker".to_string(),
            price: 129.99,
            description: "Take your music anywhere with the JBL Flip 5 – the perfect portable speaker for any adventure! With powerful sound, waterproof design, and up to 12 hours of playtime, it's built to keep the music going, rain or shine.".to_string(),
            image: "/JBLFlip5WaterproofBluetoothWirelessSpeaker.jpg".to_string()
        },
        Product {
            id: 9,
            name: "Macbook Air".to_string(),
            price: 1649.99,
            description: "Experience the ultimate in sleek design and high performance with the MacBook Air. Powered by the M2 chip, this ultra-lightweight laptop delivers incredible speed, long-lasting battery life, and all the power you need to tackle any task—whether you're working, studying, or creating.".to_string(),
            image: "/macbookair.jpg".to_string()
        },
        Product {
            id: 10,
            name: "Metaphor-Refantazio PS5".to_string(),
            price: 79.99,
            description: "Dive into a world of fantasy, mystery, and strategic combat with Metaphor: ReFantazio, the latest RPG from the creators of Shin Megami Tensei! Explore a beautifully crafted world, meet unforgettable characters, and uncover dark secrets in this epic tale.".to_string(),
            image: "/metaphor-refantazio-ps5.jpg".to_string()
        }
    ]
}