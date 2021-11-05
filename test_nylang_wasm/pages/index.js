import styles from '../styles/Home.module.css'
import { useState } from 'react'
import { excute_nyl } from "../../pkg/nylang_bg"

const MODE = {
  ast: 0,
  run: 1
}

const INITIAL_SC = `🍙 main = 🏨 ( ) { 
  🎤 ( "hello nylang" ) ;
  🍙 calc = 3 + 5 ;
  🎤🎶 ( "3 + 5 = " + calc ) ;
  🎤 ( "lets print 'doggy' 10 times using loop function:") ;
  🌸 (
    🏨 () {
      🎤 ( "doggy" ) ;
    },
    5
  )
} ; 
`

export default function Home() {
  const [nyl, setNyl] = useState(INITIAL_SC)
  const [ast, setAst] = useState(excute_nyl(INITIAL_SC, MODE.ast))
  const [outouts, setOutouts] = useState(excute_nyl(INITIAL_SC, MODE.run))
  return (
    <div className={styles.container}>
      <div className={styles.half}>
        nylang text:
        <button onClick={
          () => {
            try {
              setAst(excute_nyl(nyl, MODE.ast))
              setOutouts(excute_nyl(nyl, MODE.run))
            } catch (_) {
              alert("excution failed")
            }
          }
        }>excute</button>
        <textarea
          className={styles.md_area}
          value={nyl}
          onChange={(e) => setNyl(e.target.value)}
        />
      </div>
      <div className={styles.half}>
        <div>
          <h1> ast </h1>
          {
            ast.map((ast, i) => {
              return (
                <div key={i} className={styles.line}>
                  {i}: {ast}
                  <hr/>
                </div>
              )
            })
          }

          <h1> excute </h1>
          {
            outouts.map((outout, i) => {
              return (
                <div key={i} className={styles.line}>
                  {i}: {outout}
                  <hr/>
                </div>
              )
            })
          }
        </div>
      </div>
    </div>
  )
}
