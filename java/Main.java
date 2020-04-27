// Main.java
import java.util.*;
public class Main {
  public static void main(String[] args) {
    long start = System.currentTimeMillis();
    Map<Integer, Integer> m = new HashMap<Integer, Integer>();
    for (int i = 0; i < 10000; i++) {
      m.put(i, i);
    }
    int ans = 0;
    for (Integer key : m.keySet()) {
      ans += m.get(key);
    }
    long end = System.currentTimeMillis();
    System.out.printf("answer=%d, %d ms", ans, end - start);
  }
}
