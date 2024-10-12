/**
 * Copyright 2024 eternal-flame-AD <yume@yumechi.jp>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 * SPDX-License-Identifier: Apache-2.0
 */

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