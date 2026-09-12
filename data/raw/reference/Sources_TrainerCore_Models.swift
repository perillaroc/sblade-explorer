import Foundation

public enum TrainerCategory: String, CaseIterable, Identifiable, Sendable {
    case currency = "货币"
    case material = "材料"
    case core = "核心升级"
    case skill = "技能"
    case ammunition = "弹药"
    case consumable = "消耗品"

    public var id: String { rawValue }

    public var symbol: String {
        switch self {
        case .currency: "creditcard.fill"
        case .material: "cube.fill"
        case .core: "cpu.fill"
        case .skill: "sparkles"
        case .ammunition: "scope"
        case .consumable: "cross.case.fill"
        }
    }
}

public enum SaveField: Hashable, Sendable {
    case inventory(alias: String)
    case intProperty(name: String)
    case incrementingIntProperty(name: String)

    public var isIncrementing: Bool {
        if case .incrementingIntProperty = self { return true }
        return false
    }
}

public struct TrainerDefinition: Identifiable, Hashable, Sendable {
    public let id: String
    public let title: String
    public let subtitle: String
    public let category: TrainerCategory
    public let field: SaveField
    public let suggestedValue: Int64

    public init(
        id: String,
        title: String,
        subtitle: String,
        category: TrainerCategory,
        field: SaveField,
        suggestedValue: Int64
    ) {
        self.id = id
        self.title = title
        self.subtitle = subtitle
        self.category = category
        self.field = field
        self.suggestedValue = suggestedValue
    }
}

public extension TrainerDefinition {
    static let builtIn: [TrainerDefinition] = [
        .init(id: "vitcoin", title: "维特币", subtitle: "ETC_Item_VendingMachineCoin", category: .currency, field: .inventory(alias: "ETC_Item_VendingMachineCoin"), suggestedValue: 9_999),
        .init(id: "gold", title: "金币", subtitle: "BetaCrystal", category: .currency, field: .inventory(alias: "BetaCrystal"), suggestedValue: 99_999),
        .init(id: "stellar-tear", title: "星之泪", subtitle: "Money_Nier", category: .currency, field: .inventory(alias: "Money_Nier"), suggestedValue: 9_999),
        .init(id: "bone-wrench", title: "骨头扳手", subtitle: "Money_Nikke", category: .currency, field: .inventory(alias: "Money_Nikke"), suggestedValue: 9_999),

        .init(id: "low-wafer", title: "纳米材料（低密度）", subtitle: "LowDensityWafer", category: .material, field: .inventory(alias: "LowDensityWafer"), suggestedValue: 99_999),
        .init(id: "high-wafer", title: "纳米材料（高密度）", subtitle: "HighDensityWafer", category: .material, field: .inventory(alias: "HighDensityWafer"), suggestedValue: 99_999),
        .init(id: "quantum-wafer", title: "纳米材料（二维量子）", subtitle: "2DQuantumWafer", category: .material, field: .inventory(alias: "2DQuantumWafer"), suggestedValue: 99_999),
        .init(id: "elastomer", title: "聚合材料（弹性纤维）", subtitle: "ElastomerFiber", category: .material, field: .inventory(alias: "ElastomerFiber"), suggestedValue: 99_999),
        .init(id: "film", title: "聚合材料（有机薄膜）", subtitle: "PolymerOrganicFilm", category: .material, field: .inventory(alias: "PolymerOrganicFilm"), suggestedValue: 99_999),
        .init(id: "rayon", title: "聚合材料（纤维素）", subtitle: "ECelluloseRayon", category: .material, field: .inventory(alias: "ECelluloseRayon"), suggestedValue: 99_999),
        .init(id: "micro-drive", title: "微型驱动装置", subtitle: "Money_JunkIron_A", category: .material, field: .inventory(alias: "Money_JunkIron_A"), suggestedValue: 99_999),
        .init(id: "micro-motor", title: "微型马达", subtitle: "Money_JunkIron_B", category: .material, field: .inventory(alias: "Money_JunkIron_B"), suggestedValue: 99_999),
        .init(id: "micro-coil", title: "微型线圈", subtitle: "Money_JunkIron_C", category: .material, field: .inventory(alias: "Money_JunkIron_C"), suggestedValue: 99_999),

        .init(id: "body-core", title: "机体核心", subtitle: "BodyCore", category: .core, field: .inventory(alias: "BodyCore"), suggestedValue: 99),
        .init(id: "beta-core", title: "贝塔核心", subtitle: "betacore", category: .core, field: .inventory(alias: "betacore"), suggestedValue: 99),
        .init(id: "weapon-core", title: "武器核心", subtitle: "WeaponCore", category: .core, field: .inventory(alias: "WeaponCore"), suggestedValue: 10),
        .init(id: "damaged-weapon-core", title: "受损的武器核心", subtitle: "WeaponCore_Damaged", category: .core, field: .inventory(alias: "WeaponCore_Damaged"), suggestedValue: 99),
        .init(id: "drone-core", title: "无人机升级模块", subtitle: "DroneCore", category: .core, field: .inventory(alias: "DroneCore"), suggestedValue: 99),
        .init(id: "gear-core", title: "万用螺栓", subtitle: "GearCore", category: .core, field: .inventory(alias: "GearCore"), suggestedValue: 99),
        .init(id: "tumbler-core", title: "维生包扩展模块", subtitle: "TumblerCore", category: .core, field: .inventory(alias: "TumblerCore"), suggestedValue: 99),

        .init(id: "skill-experience", title: "技能经验值", subtitle: "SPExp · 推荐 99000，进游戏后获取一次经验触发结算", category: .skill, field: .intProperty(name: "SPExp"), suggestedValue: 99_000),

        .init(id: "bullet-normal", title: "普通弹药", subtitle: "Bullet_Normal", category: .ammunition, field: .inventory(alias: "Bullet_Normal"), suggestedValue: 999),
        .init(id: "bullet-scatter", title: "散弹", subtitle: "Bullet_Scatter", category: .ammunition, field: .inventory(alias: "Bullet_Scatter"), suggestedValue: 999),
        .init(id: "bullet-missile", title: "导弹", subtitle: "Bullet_Missile", category: .ammunition, field: .inventory(alias: "Bullet_Missile"), suggestedValue: 999),
        .init(id: "bullet-railgun", title: "轨道炮弹药", subtitle: "Bullet_Railgun", category: .ammunition, field: .inventory(alias: "Bullet_Railgun"), suggestedValue: 999),

        .init(id: "small-potion", title: "小型恢复药", subtitle: "HP_Potion_Small", category: .consumable, field: .inventory(alias: "HP_Potion_Small"), suggestedValue: 99),
        .init(id: "potion", title: "恢复药", subtitle: "HP_Potion", category: .consumable, field: .inventory(alias: "HP_Potion"), suggestedValue: 99),
        .init(id: "recovery-potion", title: "持续恢复药", subtitle: "Recovery_HP_Potion", category: .consumable, field: .inventory(alias: "Recovery_HP_Potion"), suggestedValue: 99),
        .init(id: "concussion", title: "震荡手榴弹", subtitle: "ConcussionGrenade", category: .consumable, field: .inventory(alias: "ConcussionGrenade"), suggestedValue: 99),
        .init(id: "pulse", title: "脉冲手榴弹", subtitle: "PulseGrenade", category: .consumable, field: .inventory(alias: "PulseGrenade"), suggestedValue: 99),
    ]
}

public struct ParsedInventoryItem: Sendable {
    public let alias: String
    public let count: UInt32
    public let chargeCount: UInt32?
    public let countOffset: Int
    public let chargeCountOffset: Int?
}

public struct ParsedIntProperty: Sendable {
    public let name: String
    public let value: Int32
    public let valueOffset: Int
}

public struct SaveSnapshot: Sendable {
    public let saveURL: URL
    public let modifiedAt: Date
    public let inventory: [String: ParsedInventoryItem]
    public let intProperties: [String: ParsedIntProperty]
    public let isCRCValid: Bool

    public func value(for field: SaveField) -> Int64? {
        switch field {
        case .inventory(let alias): inventory[alias].map { Int64($0.count) }
        case .intProperty(let name), .incrementingIntProperty(let name):
            intProperties[name].map { Int64($0.value) }
        }
    }
}

public enum SaveChange: Sendable {
    case inventory(alias: String, value: UInt32)
    case intProperty(name: String, value: Int32)
}

public enum TrainerError: LocalizedError {
    case saveNotFound
    case invalidSave(String)
    case fieldMissing(String)
    case gameIsRunning
    case verificationFailed(String)

    public var errorDescription: String? {
        switch self {
        case .saveNotFound: "没有找到 StellarBladeSave00.sav"
        case .invalidSave(let message): "存档格式无效：\(message)"
        case .fieldMissing(let field): "存档中没有找到 \(field)"
        case .gameIsRunning: "《剑星》正在运行，请先保存并退出游戏"
        case .verificationFailed(let message): "写入验证失败：\(message)"
        }
    }
}
