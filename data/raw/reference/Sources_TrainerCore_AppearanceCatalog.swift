import Foundation

public enum AppearanceCategory: String, CaseIterable, Identifiable, Sendable {
    case hair = "伊芙发型"
    case faceAccessory = "眼镜/面饰"
    case earring = "耳饰"
    case adam = "亚当服饰"
    case lily = "莉莉服饰"
    case drone = "无人机外观"

    public var id: String { rawValue }

    public var symbol: String {
        switch self {
        case .hair: "comb.fill"
        case .faceAccessory: "eyeglasses"
        case .earring: "diamond.fill"
        case .adam: "person.fill"
        case .lily: "person.crop.circle.fill"
        case .drone: "dot.scope"
        }
    }
}

public struct AppearanceDefinition: Identifiable, Hashable, Sendable {
    public let alias: String
    public let name: String
    public let category: AppearanceCategory

    public var id: String { alias }
}

public enum AppearanceCatalog {
    /// Names are taken from the Simplified Chinese localization shipped with the game.
    public static let all: [AppearanceDefinition] = [
        .init(alias: "Hair_000", name: "星球空降马尾", category: .hair),
        .init(alias: "Hair_001", name: "卡西姆的抉择", category: .hair),
        .init(alias: "Hair_002", name: "最可爱的发型", category: .hair),
        .init(alias: "Hair_003", name: "希雍公主", category: .hair),
        .init(alias: "Hair_004", name: "桃香", category: .hair),
        .init(alias: "Hair_005", name: "哥特时代", category: .hair),
        .init(alias: "Hair_006", name: "女儿的回忆", category: .hair),
        .init(alias: "Hair_007", name: "初恋", category: .hair),
        .init(alias: "Hair_008", name: "武士刀", category: .hair),
        .init(alias: "Hair_009", name: "情人节", category: .hair),
        .init(alias: "Hair_010", name: "卡西姆的招牌发型", category: .hair),
        .init(alias: "Hair_011", name: "假日", category: .hair),
        .init(alias: "Hair_012", name: "创新之作", category: .hair),
        .init(alias: "Hair_Christmas_01", name: "圣诞女孩", category: .hair),
        .init(alias: "FaceAccessory_001", name: "椭圆角质架眼镜", category: .faceAccessory),
        .init(alias: "FaceAccessory_002", name: "大圆框眼镜", category: .faceAccessory),
        .init(alias: "FaceAccessory_002_Var2", name: "超大圆框眼镜", category: .faceAccessory),
        .init(alias: "FaceAccessory_003", name: "经典圆框眼镜", category: .faceAccessory),
        .init(alias: "FaceAccessory_004", name: "棕色角质架眼镜", category: .faceAccessory),
        .init(alias: "FaceAccessory_005", name: "金属框架眼镜", category: .faceAccessory),
        .init(alias: "FaceAccessory_006", name: "猫眼", category: .faceAccessory),
        .init(alias: "FaceAccessory_007", name: "实验室护目镜", category: .faceAccessory),
        .init(alias: "FaceAccessory_008", name: "超窄太阳镜", category: .faceAccessory),
        .init(alias: "FaceAccessory_009", name: "橙色飞行员太阳镜", category: .faceAccessory),
        .init(alias: "FaceAccessory_010", name: "多边形方框眼镜", category: .faceAccessory),
        .init(alias: "FaceAccessory_011", name: "方框眼镜", category: .faceAccessory),
        .init(alias: "FaceAccessory_012", name: "半框眼镜", category: .faceAccessory),
        .init(alias: "FaceAccessory_013", name: "超大号太阳镜", category: .faceAccessory),
        .init(alias: "FaceAccessory_026", name: "环绕式太阳镜", category: .faceAccessory),
        .init(alias: "FaceAccessory_027", name: "漩涡眼镜", category: .faceAccessory),
        .init(alias: "FaceAccessory_028", name: "眼罩", category: .faceAccessory),
        .init(alias: "FaceAccessory_029", name: "偏光太阳镜", category: .faceAccessory),
        .init(alias: "FaceAccessory_Christmas_01", name: "冰晶眼镜", category: .faceAccessory),
        .init(alias: "Earring_001", name: "绯红泪珠", category: .earring),
        .init(alias: "Earring_001_Var2", name: "高贵泪珠", category: .earring),
        .init(alias: "Earring_002", name: "点点蓝晶", category: .earring),
        .init(alias: "Earring_002_Var2", name: "光点", category: .earring),
        .init(alias: "Earring_003", name: "银齿", category: .earring),
        .init(alias: "Earring_003_Var2", name: "金齿", category: .earring),
        .init(alias: "Earring_004", name: "聚变之光", category: .earring),
        .init(alias: "Earring_005", name: "护耳", category: .earring),
        .init(alias: "Earring_006", name: "绿松石环", category: .earring),
        .init(alias: "Earring_007", name: "太阳之泪", category: .earring),
        .init(alias: "Earring_008", name: "记忆耳坠", category: .earring),
        .init(alias: "Earring_008_Var2", name: "闪亮记忆", category: .earring),
        .init(alias: "Earring_009", name: "辐射钟摆", category: .earring),
        .init(alias: "Earring_010", name: "黑色矩形", category: .earring),
        .init(alias: "Earring_011", name: "四倍矩形", category: .earring),
        .init(alias: "Earring_012", name: "黄金之心", category: .earring),
        .init(alias: "Earring_013", name: "美玉良珀", category: .earring),
        .init(alias: "Earring_014", name: "水晶之泪", category: .earring),
        .init(alias: "Earring_015", name: "红宝石之泪", category: .earring),
        .init(alias: "Earring_016", name: "摇摆兔兔", category: .earring),
        .init(alias: "Earring_017", name: "黄金枝", category: .earring),
        .init(alias: "Earring_018", name: "承传标记", category: .earring),
        .init(alias: "Earring_Christmas_01", name: "花环耳饰", category: .earring),
        .init(alias: "Earring_Christmas_02", name: "雪橇耳夹", category: .earring),

        .init(alias: "AdamCostume_001", name: "下水道老鼠", category: .adam),
        .init(alias: "AdamCostume_002", name: "占星师服装", category: .adam),
        .init(alias: "AdamCostume_003", name: "拾荒人", category: .adam),
        .init(alias: "AdamCostume_003_Var2", name: "废品回收人", category: .adam),
        .init(alias: "AdamCostume_004", name: "变色龙", category: .adam),
        .init(alias: "AdamCostume_004_Var2", name: "夜鹰", category: .adam),
        .init(alias: "AdamCostume_Christmas_01", name: "我不是圣诞老人", category: .adam),
        .init(alias: "LilyCostume_001", name: "阿尔忒弥斯", category: .lily),
        .init(alias: "LilyCostume_002", name: "占星师外套", category: .lily),
        .init(alias: "LilyCostume_003", name: "忧郁之雨", category: .lily),
        .init(alias: "LilyCostume_003_Var2", name: "雨天", category: .lily),
        .init(alias: "LilyCostume_004", name: "休假日", category: .lily),
        .init(alias: "LilyCostume_004_Var2", name: "休息日", category: .lily),
        .init(alias: "DroneSeal_001", name: "战术包", category: .drone),
        .init(alias: "DroneSeal_002", name: "占星师套装", category: .drone),
        .init(alias: "DroneSeal_002_Var2", name: "铁甲套装", category: .drone),
        .init(alias: "DroneSeal_003", name: "装甲包", category: .drone),
        .init(alias: "DroneSeal_004", name: "兔宝宝套装", category: .drone),
        .init(alias: "DroneSeal_004_Var2", name: "迷你兔套装", category: .drone),
        .init(alias: "DroneSeal_005", name: "毛绒熊熊套装", category: .drone),
        .init(alias: "DroneSeal_Christmas_01", name: "驯鹿鲁道夫套装", category: .drone),
    ]
}

public enum DLCShopCosmetics {
    /// Inventory aliases written by v0.7 that can bypass the DLC shop's own purchase registration.
    public static let aliases = [
        "BS_Nier_01", "BS_Nier_02", "BS_Nier_03", "BS_Nier_04",
        "BS_Nikke_01", "BS_Nikke_02", "BS_Nikke_03", "BS_Nikke_04", "BS_Nikke_05", "BS_Nikke_06",
        "Hair_Nier_001", "Hair_Nier_002", "Hair_Nier_003",
        "Hair_Nikke_01", "Hair_Nikke_02", "Hair_Nikke_03", "Hair_Nikke_04", "Hair_Nikke_05", "Hair_Nikke_06",
        "FaceAccessory_Nier_001",
        "AdamCostume_Nier_001", "LilyCostume_Nier_001", "DroneSeal_Nier_001",
    ]
}
