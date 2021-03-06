import * as React from 'react';

import Day1 from './Day1';
import Day2 from './Day2';
import Day3 from './Day3';
import Day4 from './Day4';
import Day5 from './Day5';

class App extends React.Component<{}, { apiMessage: string }> {
  constructor(props: object) {
    super(props);

    this.state = { apiMessage: "Loading... (If this takes too long, the database might be down.)" };
  }

  public componentDidMount() {
    fetch("/api")
      .then(r => r.status === 500
        ? `(The server reported an error or cannot be reached. Is it compiling...?)`
        : r.text()
      )
      .then(apiMessage =>
        this.setState({
          apiMessage
        })
      );
  }

  public render() {
    return (
      <div className="p-3 container">
        <div className="jumbotron">
          <h1 className="header">
            Matt's Advent of Code 2020
          </h1>
        </div>

        <Day5/>
        <Day4/>
        <Day3/>
        <Day2/>
        <Day1/>

        <h2>Status</h2>
        <p>
          {this.state.apiMessage}
        </p>
      </div>
    );
  }
}

export default App;
