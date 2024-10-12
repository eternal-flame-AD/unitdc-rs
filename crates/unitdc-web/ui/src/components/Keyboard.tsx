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

import { useRef, useState } from "react"

export type TokenType = "operator" | "literal_num" | "unit"
export type UiAction = "append_space" | "append_newline" | "backspace" | "submit" | "clear"

export interface KeyboardProps {
    onUiAction: (action: UiAction) => void
    onToken: (token: string, tokentype: TokenType) => void
}

export function Keyboard(props: KeyboardProps) {
    let ref = useRef<HTMLDivElement>(null)
    let [mounted, setMounted] = useState(false);
    let [modifier_pressed, setModifierPressed] = useState("");

    if (!mounted) {
        setMounted(true);
    }

    function ModifierButton(props: { modifier: string }) {
        return (
            <div
                className={"keyboard-key" + (modifier_pressed == props.modifier ? " modifier-pressed" : "")}
                data-tokentype="unit_modifier"
                onClick={() => setModifierPressed(modifier_pressed == props.modifier ? "" : props.modifier)}>{`(${props.modifier}*)`}</div>
        )
    }
    function UiActionButton(btn_props: { action: UiAction, text: string }) {
        return (
            <div className="keyboard-key" onClick={() => { props.onUiAction(btn_props.action) }}>{btn_props.text}</div>
        )
    }
    function TokenButton(btn_props: { token: string, tokentype: TokenType, text?: string }) {
        let token = btn_props.token;
        if (btn_props.tokentype == "unit") {
            token = modifier_pressed + btn_props.token;
        }
        return (
            <div className="keyboard-key" data-tokentype={btn_props.tokentype}
                onClick={() => {
                    setModifierPressed("")
                    props.onToken(token, btn_props.tokentype)
                }}>{btn_props.text || btn_props.token}</div>
        )
    }

    return (
        <div className="unitdc-keyboard" id="unitdc-keyboard" ref={ref}>

            <div className="keyboard-col">
                <div className="keyboard-key" data-tokentype="operator">c</div>
                {
                    ["k", "c", "d"].map((modifier) => {
                        return (
                            <ModifierButton modifier={modifier} key={modifier} />
                        )
                    })
                }
                <UiActionButton action="append_space" text="␣" />
            </div>
            <div className="keyboard-col">
                <div className="keyboard-key" data-tokentype="operator">d</div>
                {
                    ["m", "u", "n"].map((modifier) => {
                        return (
                            <ModifierButton modifier={modifier} key={modifier} />
                        )
                    })
                }

                <UiActionButton action="backspace" text="←" />
            </div>

            <div className="keyboard-col">
                <TokenButton token="v" tokentype="operator" />
                {
                    ["7", "4", "1", "."].map((token) => {
                        return (
                            <TokenButton token={token} tokentype="literal_num" key={token} />
                        )
                    })
                }
            </div>
            <div className="keyboard-col">
                <TokenButton token="p" tokentype="operator" />
                {
                    ["8", "5", "2", "0"].map((token) => {
                        return (
                            <TokenButton token={token} tokentype="literal_num" key={token} />
                        )
                    })
                }
            </div>
            <div className="keyboard-col">
                <TokenButton token="n" tokentype="operator" />
                {
                    ["9", "6", "3", "e"].map((token) => {
                        return (
                            <TokenButton token={token} tokentype="literal_num" key={token} />
                        )
                    })
                }
                <UiActionButton action="append_newline" text="↩" />
            </div>


            <div className="keyboard-col">
                {
                    ["f", "+", "-", "*", "/"].map((token) => {
                        return (
                            <TokenButton token={token} tokentype="operator" key={token} />
                        )
                    })
                }
                <UiActionButton action="submit" text="✓" />
            </div>
            <div className="keyboard-col">
                <TokenButton token="r" tokentype="operator" />
                <TokenButton token="s" tokentype="operator" />
                {
                    ["1", "g", "l", "iu"].map((token) => {
                        return (
                            <TokenButton token={token} tokentype="unit" text={`(${token})`} key={token} />
                        )
                    })
                }
            </div>
            <div className="keyboard-col">
                <UiActionButton action="clear" text="CLR" />
                <TokenButton token="U" tokentype="operator" />
                {
                    ["m", "mol", "M", "Da"].map((token) => {
                        return (
                            <TokenButton token={token} tokentype="unit" text={`(${token})`} key={token} />
                        )
                    })
                }
            </div>
        </div>
    )
}