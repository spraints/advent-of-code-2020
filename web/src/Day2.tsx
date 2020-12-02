import * as React from 'react'
import IDayProps from './DayProps'

const PASSWORD_LINE = /(\d+)-(\d+) (.): (.*)/g
const PASSWORD_LINE2 = /(\d+)-(\d+) (.): (.*)/

interface IState {
  input: string
  output: IOutput | null
}

interface IOutput {
  valid: IPasswordLine[]
  invalid: IPasswordLine[]
}

interface IPasswordLine {
  policy: IPasswordPolicy
  password: string
}

interface IPasswordPolicy {
  min: number
  max: number
  c: string
}

class Today extends React.Component<IDayProps, IState> {
  constructor(props: IDayProps) {
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
    const Res = (props: any) => (
      <div>
        <i>{props.label}</i>
        <ul>
          {props.items.map((line: IPasswordLine) => (<li>{line.policy.min}-{line.policy.max} {line.policy.c}: {line.password}</li>))}
        </ul>
      </div>
    )
    return (
      <p>
        <b>{this.state.output.valid.length}</b>
        <Res label="Valid" items={this.state.output.valid}/>
        <Res label="Not valid" items={this.state.output.invalid}/>
      </p>
    )
  }

  private solve() {
    this.setState({output: null})
    const matches = this.state.input.match(PASSWORD_LINE)
    if (matches) {
      const lines = [...matches].map((match: string) => {
        const m = match.match(PASSWORD_LINE2)
        if (!m) {
          return {}
        }
        return {
          password: m[4],
          policy: {min: parseInt(m[1], 10), max: parseInt(m[2], 10), c: m[3]}
        }
      })

      // tslint:disable:no-console
      // console.log(passwords)

      const req = {
        body: JSON.stringify({lines, part2: this.props.part2}),
        headers: {"Content-Type": "application/json"},
        method: "POST"
      }
      fetch("/api/day2", req).then(r => this.setOutput(r))
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

export default Today
