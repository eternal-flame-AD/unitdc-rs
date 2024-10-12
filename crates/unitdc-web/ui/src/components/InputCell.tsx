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

import { useEffect, useRef } from "react"

export interface InputCellProps {
    number: number,
    active?: boolean
    text: string
    onchange?: (value: string) => void
    onsubmit?: (value: string) => void
}

export function InputCell(props: InputCellProps) {
    const ref = useRef<HTMLTextAreaElement>(null);

    useEffect(() => {
        function handleKeyDown(event: KeyboardEvent) {
            if (event.key === "Enter" && event.shiftKey) {
                event.preventDefault()
                if (props.onsubmit) {
                    props.onsubmit(ref.current?.value || "")
                }
            }
        }
        if (props.active) {
            ref.current?.focus()
            document.addEventListener("keydown", handleKeyDown)
        } else {
            document.removeEventListener("keydown", handleKeyDown)
        }
        return () => {
            document.removeEventListener("keydown", handleKeyDown)
        }
    }, [props.active])

    const onInput = () => {
        ref.current!.style.height = "auto"
        ref.current!.style.height = ref.current!.scrollHeight + "px"
        if (props.onchange) {
            props.onchange(ref.current?.value || "")
        }
    }

    useEffect(() => {
        ref.current!.value = props.text
        ref.current!.addEventListener("input", onInput)
        return () => {
            ref.current!.removeEventListener("input", onInput)
        }

    }, [props.text])


    return (
        <div className={"unitdc-io input" + (props.active ? " input-active" : "")}>
            <label className="prompt">{`In [${props.number}]:`} <span className="submit-hint">Shift-Enter to Submit</span></label>
            <textarea className="input-text" ref={ref} readOnly={!props.active} />
        </div>
    )
}