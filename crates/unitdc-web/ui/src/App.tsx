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


import { useState, useReducer } from 'react'

import './App.css'
import { Keyboard, TokenType } from './components/Keyboard'
import { InputCell } from './components/InputCell';
import unitdc_wasm, { unitdc_input, unitdc_init } from '../../pkg';
import { ErrorCell } from './components/ErrorCell';
import { Quantity } from './types';
import { OutputCell } from './components/OutputCell';
import { useForceUpdate } from './util';

type IoCellDef = IOTextCellDef | IOQuantityCellDef;

interface IOQuantityCellDef {
  type: 'output',
  quantity: Quantity[]
}

interface IOTextCellDef {
  type: 'input' | 'message' | 'error',
  text: string,
}

type IoCellAction = IoCellAddAction | IoCellUpdateTextAction;

interface IoCellAddAction {
  type: 'add',
  cell: IoCellDef,
}

interface IoCellUpdateTextAction {
  type: 'updateText',
  index: number,
  text: string,
}

function IoCellReducer(state: IoCellDef[], action: IoCellAction) {
  switch (action.type) {
    case 'add':
      return [
        ...state,
        action.cell,
      ]
    case 'updateText':
      const newState = [...state];
      (newState[action.index] as IOTextCellDef).text = action.text;
      return newState;
    default:
      throw new Error();
  }
}

function App() {
  const [ioCells, ioCellsDispatch] = useReducer(IoCellReducer, []);
  const [mounted, setMounted] = useState(false);
  const [lastTokenType, setLastTokenType] = useState<TokenType | "">("");
  const forceUpdate = useForceUpdate();

  const addCells = (cells: IoCellDef[]) => {
    cells.forEach((cell) => {
      ioCellsDispatch({ type: 'add', cell: cell })
    })
  }

  const lastInputCell = () => ioCells.filter((cell) => cell.type === 'input').slice(-1)[0] as IOTextCellDef;

  const appendToken = (token: string, tokenType: TokenType) => {
    let cell = lastInputCell();
    if (lastTokenType != tokenType) {
      cell.text += ' ';
    }
    if (tokenType === 'unit') {
      token = '(' + token + ')';
    }
    cell.text += token;
    setLastTokenType(tokenType);
    forceUpdate();
  }


  const processOutput = (type: 'quantity' | 'quantity_list' | 'message', data: any) => {
    if (type === 'quantity') {
      addCells([
        {
          type: 'output',
          quantity: [data],
        }
      ])
    } else if (type === 'quantity_list') {
      addCells([
        {
          type: 'output',
          quantity: data,
        }
      ])
    } else if (type === 'message') {
      addCells([
        {
          type: 'message',
          text: data,
        }
      ])
    }
  }

  const submit = () => {
    let success = false;
    let text = lastInputCell().text;
    console.log('submit', text);
    try {
      unitdc_input(text)
      success = true;
    } catch (e) {
      addCells([
        {
          type: 'error',
          text: (e as any).toString(),
        }
      ])
    }
    addCells([
      {
        type: 'input',
        text: success ? '' : text,
      }
    ])
  };

  if (!mounted) {
    setMounted(true);
    unitdc_wasm().then(() => {
      unitdc_init(processOutput);
      addCells([
        {
          type: 'input',
          text: '',
        }
      ])
    }).catch(e => {
      addCells([
        {
          type: 'error',
          text: (e as any).toString(),
        }
      ])
    });
  }


  return (
    <>
      <div className="unitdc-container">
        <h1 style={{ whiteSpace: 'nowrap' }}>
          UnitDC
          <span style={{
            fontStyle: 'italic',
            fontWeight: 'lighter',
            fontSize: 'smaller',
          }} id="unitdc-description">Unit-aware Desk Calculator</span>
        </h1>

        <div id="unitdc-dialog">
          {
            ioCells.map((cell, index) => {
              switch (cell.type) {
                case 'output':
                  return (
                    <OutputCell quantities={cell.quantity} key={index} />
                  )
                case 'input':
                  return (
                    <InputCell
                      key={index} number={index}
                      text={cell.text} active={index == ioCells.length - 1}
                      onchange={(value) => {
                        console.log('onchange', value);
                        ioCellsDispatch({ type: 'updateText', index: index, text: value })
                      }}
                      onsubmit={submit}
                    />
                  )
                case 'error':
                  return (
                    <ErrorCell key={index} text={cell.text} />
                  )
                case 'message':
                  return (
                    <pre key={index}>{cell.text}</pre>
                  )
              }
            })
          }
        </div>

        {
          <div className="unitdc-keyboard-spacer"></div>
        }
      </div>
      {
        <Keyboard
          onToken={appendToken}
          onUiAction={(action) => {
            switch (action) {
              case 'append_space':
                lastInputCell().text += ' ';
                forceUpdate();
                break;
              case 'append_newline':
                lastInputCell().text += '\n';
                forceUpdate();
                break;
              case 'clear':
                lastInputCell().text = '';
                forceUpdate();
                break;
              case 'backspace':
                lastInputCell().text = lastInputCell().text.slice(0, -1);
                forceUpdate();
                break;
              case 'submit':
                submit();
                break;
            }
          }}
        />
      }
    </>
  )
}

export default App
