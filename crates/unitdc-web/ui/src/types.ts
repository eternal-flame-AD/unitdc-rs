export interface Quantity {
    _str: string;
    number_float: number;
    unit: UnitCombo,
}

export type UnitCombo = UnitExponent[];

export interface UnitExponent {
    unit: string;
    exponent: number;
}

export interface Unit {
    symbol: string;
}