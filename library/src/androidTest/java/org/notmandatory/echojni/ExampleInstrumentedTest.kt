package org.bitcoindevkit.bdkjni

import android.util.Log
import androidx.test.ext.junit.runners.AndroidJUnit4
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.*
import kotlinx.coroutines.runBlocking
import org.junit.Assert.*
import org.junit.Test
import org.junit.runner.RunWith
import org.notmandatory.echojni.Lib

/**
 * Instrumented test, which will execute on an Android device.
 *
 * See [testing documentation](http://d.android.com/tools/testing).
 *
 */
@RunWith(AndroidJUnit4::class)
class ExampleInstrumentedTest {

    companion object {
        init {
            System.loadLibrary("echo_jni")
        }
    }

    @Test
    fun multithread() {
        runBlocking {
            val flow1 = newFlow(1).flowOn(Dispatchers.IO)
            val flow2 = newFlow(2).flowOn(Dispatchers.IO)
            flow1.flatMapMerge(concurrency = 2) { flow2 }.collect()
            //flow1.collect()
        }
    }

    private fun newFlow(id: Int): Flow<Pair<Int, String>> {
        return (1..100).asFlow()
            .onStart { Log.d("MT-TEST", "Start flow $id") }
            .onCompletion { Log.d("MT-TEST", "Complete flow $id") }
            //.map { MT-TESTAndGetBalance(it) }
            .onEach { Log.d("MT-TEST", "Flow $id, iteration $it") }
            .map {
                Pair(it, Lib().echo("Flow $id, iteration $it"))
            }
            .catch { e ->
                Log.e("MT-TEST", "Failed flow $id with exception: $e")
                fail()
            }
            .onEach {
                //assertFalse(it.second == 0L)
                Log.d("MT-TEST", "Verifying flow $id, iteration ${it.first}")
                assertFalse(
                    "NullOrEmpty, flow $id, iteration ${it.first}",
                    it.second.isNullOrEmpty()
                )
                assertEquals("Flow $id, iteration ${it.first}", it.second)
                Log.d("MT-TEST", "Finished flow $id iteration ${it.first}")
            }
    }
}
