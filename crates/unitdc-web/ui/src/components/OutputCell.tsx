import { Quantity } from "../types";

export interface OutputCellProps {
    quantities: Quantity[]
}

export function OutputCell(props: OutputCellProps) {
    return (
        <div className="unitdc-io output">
            <label className="prompt">Out:</label>
            <div className="output-text" style={{ paddingLeft: '2em' }}>
                {props.quantities.map((q, index) => `[${props.quantities.length - 1 - index}]: ${q._str}`).reverse().join("\r\n")}
            </div>
        </div>
    )
}