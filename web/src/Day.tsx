import * as React from 'react'
import PartButtons from './PartButtons'

import './Day.css'

interface IProps {
  num: string
  children: (isPart2: boolean) => JSX.Element
}

interface IState {
  visible: boolean
  part2: boolean
}

class Day extends React.Component<IProps, IState> {
  constructor(props: IProps) {
    super(props)

    this.state = { visible: false, part2: false }

    this.show = this.show.bind(this)
    this.hide = this.hide.bind(this)
    this.setPart1 = this.setPart1.bind(this)
    this.setPart2 = this.setPart2.bind(this)
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
        <PartButtons part2={this.state.part2} setPart1={this.setPart1} setPart2={this.setPart2}/>
        {this.props.children(this.state.part2)}
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
}

export default Day
