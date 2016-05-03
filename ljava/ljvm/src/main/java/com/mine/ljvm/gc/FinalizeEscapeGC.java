package com.mine.ljvm.gc;

public class FinalizeEscapeGC {
    public static FinalizeEscapeGC SAVE_HOOK = null;

    public static void main(String[] args) throws Throwable {
        SAVE_HOOK = new FinalizeEscapeGC();

        // first gc
        SAVE_HOOK = null;
        System.gc();
        // wait 0.5s for finalize() to execute because of its low priority
        Thread.sleep(500);
        if (SAVE_HOOK != null) {
            SAVE_HOOK.isAlive();
        } else {
            System.out.println("No , I am dead!");
        }

        // second gc
        SAVE_HOOK = null;
        System.gc();
        Thread.sleep(500);
        if (SAVE_HOOK != null) {
            SAVE_HOOK.isAlive();
        } else {
            System.out.println("No , I am dead!");
        }

    }

    private void isAlive() {
        System.out.println("Yes, I am still alive");
    }

    // finalize() will be only called once.
    // Avoid to use this method, it's designed to compromise for C/C--.
    @Override
    protected void finalize() throws Throwable {
        super.finalize();
        System.out.println("finalize method executed!");
        FinalizeEscapeGC.SAVE_HOOK = this;
    }
}
