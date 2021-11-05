import styles from '../styles/Home.module.css'
import { useState } from 'react'
import { excute_nyl } from "../../pkg/nylang_bg"

const INITIAL_SC = '🍙 main = 🏨 ( ) { 🎤🎶 ( "hello nylang" ) ; } ; '

export default function Home() {

  console.log(excute_nyl(INITIAL_SC));

  const [nyl, setNyl] = useState(INITIAL_SC)
  const [outouts, setOutouts] = useState(excute_nyl(INITIAL_SC))
  return (
    <div className={styles.container}>
      <div className={styles.half}>
        nylang text:
        <button onClick={
          () => {
            setOutouts(excute_nyl(nyl))
          }
        }>excute</button>
        <textarea
          className={styles.md_area}
          value={nyl}
          onChange={(e) => setNyl(e.target.value)}
        />
      </div>
      <div>
        {/* {
          outouts.map((outout, i) => {
            return (
              <div key={i} className={styles.half}>
                {i}: {outout}
              </div>
            )
          })
        } */}
      </div>
    </div>
  )
}
