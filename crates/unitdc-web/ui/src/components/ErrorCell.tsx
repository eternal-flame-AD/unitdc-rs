export interface ErrorCellProps {
    text: string
}

export function ErrorCell(props: ErrorCellProps) {
    return (
        <div className="unitdc-io error">
            <label className="prompt">Error:</label>
            <div className="error-text">{props.text}</div>
        </div>
    )
}