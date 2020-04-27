// main.go
package main
import (
  "fmt"
  "time"
  "math/rand"
  "strings"
)
func main() {
	rand.Seed(time.Now().Unix());
	simpleHash();
	complexHash();
}

func simpleHash() {
  start := time.Now()
  m := make(map[int]int)
  for i := 0; i < 10000; i++ {
    m[i] = i
  }
  var ans int
  for k := range m {
    ans += m[k]
  }
  end := time.Now()
  fmt.Printf("[simple_hash key:int] answer=%v, %f sec\n", ans, (end.Sub(start)).Seconds())
}

func genKeys() []key {
	chars := []string{ "あ", "い", "う", "え", "お" }
	keys := []key{};
	for i := 0; i < 10000; i++ {
		k := key {
			k1: key2 {
				k1: rand.Int() % 100,
				k2: rand.Int() % 100,
			},
			k2: strings.Join([]string{chars[rand.Uint32()%5], chars[rand.Uint32()%5], chars[rand.Uint32()%5]}, ""),
		}
		keys = append(keys, k)
	}
	return keys
}

type key struct {
	k1 key2
	k2 string
}

type key2 struct {
	k1 int
	k2 int
}

func complexHash() {
	keys := genKeys()
	start := time.Now()
	m := make(map[key]key)
	for _, k := range keys {
		m[k] = k
	}
	var ans int
	for k := range m {
		ans += m[k].k1.k2
	}
	end := time.Now()
	fmt.Printf("[complex_hash key:int/int/stringな構造体] answer=%v, %f sec\n", ans, (end.Sub(start)).Seconds())
}
