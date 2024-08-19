package flow

import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject

import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.checkpoint.Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.testcase.TestCase
import com.kms.katalon.core.testdata.TestData
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows

import internal.GlobalVariable
import groovy.json.JsonSlurper
import com.kms.katalon.core.util.KeywordUtil
import org.apache.commons.lang.RandomStringUtils as RandomStringUtils
import org.assertj.core.api.Assertions as Assertions

/*custreq_id = (long) Math.floor(Math.random() * 9_000_000_000L) + 1_000_000_000L
 custloan_id = (long) Math.floor(Math.random() * 9_000_000_000L) + 1_000_000_000L
 println(custloan_id)
 response1 = WS.sendRequest(findTestObject('EKYC Flow/Customer Risk Rating', [('cust_req_id') : custreq_id, ('cust_loan_id') : custloan_id]))
 println(response1.getResponseBodyContent())
 Assertions.assertThat(response1.getStatusCode()).isEqualTo(200)
 println(WS.verifyElementPropertyValue(response1, 'evaluationStatus', 'Approve')) 
 println("**************CUSTOMER RISK RATING PASSED**************") */
class ekyc_flow {
	private def response1
	def custreq_id
	def custloan_id
	//custloan_id = (long) Math.floor(Math.random() * 9_000_000_000L) + 1_000_000_000L

	def CustomerRiskRating() {
		custreq_id = (long) Math.floor(Math.random() * 9_000_000_000L) + 1_000_000_000L
		custloan_id = (long) Math.floor(Math.random() * 9_000_000_000L) + 1_000_000_000L
		println(custloan_id)
		response1 = WS.sendRequest(findTestObject('EKYC Flow/Customer Risk Rating', [('cust_req_id') : custreq_id, ('cust_loan_id') : custloan_id]))

		println(response1.getResponseBodyContent())

		Assertions.assertThat(response1.getStatusCode()).isEqualTo(200)

		println(WS.verifyElementPropertyValue(response1, 'evaluationStatus', 'Approve'))
	}
}

