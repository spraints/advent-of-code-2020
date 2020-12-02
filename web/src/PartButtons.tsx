import * as React from 'react'

interface IProps {
  part2: boolean
  setPart1: () => void
  setPart2: () => void
}

function PartButtons(props: IProps) {
  if (props.part2) {
    return (
      <div>
        <button onClick={props.setPart1}>Part 1</button> | <b>Part 2</b>
      </div>
    )
  } else {
    return (
      <div>
        <b>Part 1</b> | <button onClick={props.setPart2}>Part 2</button>
      </div>
    )
  }
}

export default PartButtons
