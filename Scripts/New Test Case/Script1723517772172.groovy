import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import org.assertj.core.api.Assertions as Assertions

/*  statreq_id = (long) Math.floor(Math.random() * 9_000_000_000L) + 1_000_000_000L
 custloan_id = (long) Math.floor(Math.random() * 9_000_000_000L) + 1_000_000_000L
 println(custloan_id)
 response4 = WS.sendRequest(findTestObject('EKYC Flow/Member Onboarding Status', [('stat_req_id') : statreq_id, ('NRIC'): IDV_Nric]))
 println(response4.getResponseBodyContent())
 Assertions.assertThat(response4.getStatusCode()).isEqualTo(200) 
 println(WS.verifyElementPropertyValue(response4, 'exists', 'true'))*/
loanreq_id = (((Math.floor(Math.random() * 9000000000)) as long) + 1000000000)

loan_id = (((Math.floor(Math.random() * 9000000000)) as long) + 1000000000)

email = ((Math.random() * 100) as int)

BRN = (((Math.floor(Math.random() * 9000000000)) as long) + 1000000000)

println(loan_id)

println(loanreq_id)

println(BRN)

Loanresponse = WS.sendRequest(findTestObject('EKYC Flow/Loan Onboarding', [('Token') : GlobalVariable.Token, ('loan_id') : loan_id
            , ('loan_req_id') : loanreq_id, ('email') : email, ('BRN') : BRN]))

println(Loanresponse.getResponseBodyContent())

Assertions.assertThat(Loanresponse.getStatusCode()).isEqualTo(200)

println(WS.verifyElementPropertyValue(Loanresponse, 'status', 'PROCESSING'))

WS.sendRequest(findTestObject('EKYC Flow/Loan Creation Status Check', [('Token') : GlobalVariable.Token, ('loan_id') : 0]))

