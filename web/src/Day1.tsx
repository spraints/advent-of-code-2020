import * as React from 'react'

const NUMBERS = /\d+/g

interface IState {
  input: string
  output: IOutput | null
}

interface IOutput {
  a: number
  b: number
  result: number
}

class Day1 extends React.Component<{}, IState> {
  constructor(props: any) {
    super(props)
    this.state = { input: '', output: null }
    this.solve = this.solve.bind(this)
    this.inputChanged = this.inputChanged.bind(this)
  }

  public render() {
    return (
      <div className="row">
        <div className="col">
          <h3>Input</h3>
          <textarea rows={10} onChange={this.inputChanged} value={this.state.input}/><br/>
          <button onClick={this.solve}>Solve!</button>
        </div>
        <div className="col">
          <h3>Result</h3>
          {this.renderResult()}
        </div>
      </div>
    )
  }

  public renderResult() {
    if (this.state.output == null) {
      return ''
    }
    return (
      <p>
        {this.state.output.a} * {this.state.output.b} = {this.state.output.result}
      </p>
    )
  }

  private solve() {
    this.setState({output: null})
    const matches = this.state.input.match(NUMBERS)
    if (matches) {
      const numbers = [...matches].map((match: any) => parseInt(match, 10))
      const req = {
        body: JSON.stringify({numbers}),
        headers: {"Content-Type": "application/json"},
        method: "POST"
      }
      fetch("/api/day1", req).then(r => this.setOutput(r))
    }
  }

  private setOutput(r: any) {
    if (r.status !== 200) {
      return
    }
    r.json().then((output: IOutput) => this.setState({output}))
  }

  private inputChanged(ev: React.ChangeEvent<HTMLTextAreaElement>) {
    this.setState({input: ev.target.value})
  }
}

export default Day1
