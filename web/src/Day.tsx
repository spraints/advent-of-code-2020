import * as React from 'react'
import PartButtons from './PartButtons'

import './Day.css'

interface IProps {
  num: string
  children: (isPart2: boolean, input: string) => JSX.Element
}

interface IState {
  visible: boolean
  part2: boolean
  input: string
}

class Day extends React.Component<IProps, IState> {
  constructor(props: IProps) {
    super(props)

    this.state = { visible: false, part2: false, input: '' }

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
            {this.props.children(this.state.part2, this.state.input)}
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
  }

  private setPart2() {
    this.setState({part2: true})
  }

  private inputChanged(ev: React.ChangeEvent<HTMLTextAreaElement>) {
    this.setState({input: ev.target.value})
  }
}

export default Day
