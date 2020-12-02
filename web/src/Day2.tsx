import * as React from 'react'

const PASSWORD_LINE = /(\d+)-(\d+) (.): (.*)/g
const PASSWORD_LINE2 = /(\d+)-(\d+) (.): (.*)/

interface IState {
  part2: boolean
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

class Today extends React.Component<{}, IState> {
  constructor(props: any) {
    super(props)
    this.state = { part2: false, input: '', output: null }
    this.solve = this.solve.bind(this)
    this.setPart1 = this.setPart1.bind(this)
    this.setPart2 = this.setPart2.bind(this)
    this.inputChanged = this.inputChanged.bind(this)
  }

  public render() {
    const PartButtons = () => this.state.part2 ? (
      <div>
        <button onClick={this.setPart1}>Part 1</button> | <b>Part 2</b>
      </div>
    ) : (
      <div>
        <b>Part 1</b> | <button onClick={this.setPart2}>Part 2</button>
      </div>
    )
    return (
      <div className="row">
        <div className="col">
          <h3>Input</h3>
          <PartButtons/>
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
        body: JSON.stringify({lines, part2: this.state.part2}),
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

  private setPart1() {
    this.setState({part2: false})
  }

  private setPart2() {
    this.setState({part2: true})
  }

  private inputChanged(ev: React.ChangeEvent<HTMLTextAreaElement>) {
    this.setState({input: ev.target.value})
  }
}

export default Today
