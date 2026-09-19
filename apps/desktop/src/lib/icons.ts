import {
  CupSoda,
  Fish,
  Gem,
  Glasses,
  KeyRound,
  LayoutDashboard,
  Medal,
  Package,
  Palette,
  PersonStanding,
  Scissors,
  ScrollText,
  Shirt,
  Sword,
  Tent,
  UserRound,
  type LucideIcon,
} from "@lucide/vue";

const CATEGORY_ICONS: Record<string, LucideIcon> = {
  nano_suits: Shirt,
  cans: CupSoda,
  records: ScrollText,
  passcodes: KeyRound,
  camps: Tent,
  hair: Scissors,
  glasses: Glasses,
  earrings: Gem,
  drone_seals: Medal,
  adam_costumes: UserRound,
  lily_costumes: PersonStanding,
  design_patterns: Palette,
  fish: Fish,
};

export const SUMMARY_ICON = LayoutDashboard;
export const APP_ICON = Sword;

export function categoryIcon(key: string): LucideIcon {
  return CATEGORY_ICONS[key] ?? Package;
}
