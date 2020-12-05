import * as React from 'react'
import PartButtons from './PartButtons'

import './Day.css'

interface IProps {
  num: string
  url: string
  parseInput: (input: string) => any
  children: (output: any) => JSX.Element
}

interface IState {
  visible: boolean
  part2: boolean
  input: string
  output: any
  error: string | null
}

class DayNG extends React.Component<IProps, IState> {
  constructor(props: IProps) {
    super(props)

    this.state = { visible: false, part2: false, input: '', output: null, error: null }

    this.show = this.show.bind(this)
    this.hide = this.hide.bind(this)
    this.setPart1 = this.setPart1.bind(this)
    this.setPart2 = this.setPart2.bind(this)
    this.inputChanged = this.inputChanged.bind(this)
  }

  public render() {
    if (!this.state.visible) {
      return (
        <div className="Day-row Day-row-collapsed container">
          <div className="row" onClick={this.show}>
            <h2>Day {this.props.num}</h2>
          </div>
        </div>
      )
    }
    return (
      <div className="Day-row Day-row-expanded container">
        <div className="row" onClick={this.hide}>
          <h2>Day {this.props.num}</h2>
        </div>
        <div className="row">
          <div className="col">
            <h3>Input</h3>
            <PartButtons part2={this.state.part2} setPart1={this.setPart1} setPart2={this.setPart2}/>
            <textarea rows={10} onChange={this.inputChanged} value={this.state.input}/><br/>
          </div>
          <div className="col">
            <h3>Output</h3>
            {this.state.error || (this.state.output == null ? '' : this.props.children(this.state.output))}
          </div>
        </div>
      </div>
    )
  }

  private show() {
    this.setState({visible: true})
  }

  private hide() {
    this.setState({visible: false})
  }

  private setPart1() {
    this.setState({part2: false})
    this.solve({input: this.state.input, part2: false})
  }

  private setPart2() {
    this.setState({part2: true})
    this.solve({input: this.state.input, part2: true})
  }

  private inputChanged(ev: React.ChangeEvent<HTMLTextAreaElement>) {
    const input = ev.target.value
    this.setState({input})
    this.solve({input, part2: this.state.part2})
  }

  private async solve(arg: {input: string, part2: boolean}) {
    const {input, part2} = arg
    if (input === '') {
      this.setState({output: null})
      return
    }

    try {
      const resp = await fetch(this.props.url, {
        body: JSON.stringify({input: this.props.parseInput(input), part2}),
        headers: {"Content-Type": "application/json"},
        method: "POST"
      })
      if (resp.status !== 200) {
        try {
          const body = await resp.text()
          this.setState({error: `${resp.status}: ${body}`})
        } catch {
          this.setState({error: `${resp.status}!`})
        }
        return
      }
      const output = await resp.json()
      this.setState({output, error: null})
    } catch(error) {
      this.setState({error: `${error}`})
    }
  }
}

///

function parseDayInput(input: string): any {
  return input
}

interface IOutputProps {
  output: IOutput
}

interface IOutput {
  valid: number
  failures: string[]
}

function Output({output}: IOutputProps) {
  return (
    <div>
      <code>{JSON.stringify(output)}</code>
    </div>
  )
}

///

export default function() {
  return (
    <DayNG num="5" url="/api/day5" parseInput={parseDayInput}>
      {(output: IOutput) => <Output output={output}/>}
    </DayNG>
  )
}
