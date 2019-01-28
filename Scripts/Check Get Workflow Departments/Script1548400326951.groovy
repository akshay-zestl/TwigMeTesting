import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import groovy.json.JsonSlurper as JsonSlurper
import static org.assertj.core.api.Assertions.*

def DEPARTMENTS = GlobalVariable.DEPARTMENTS
for (def index : (0..DEPARTMENTS.size())) {
	if(DEPARTMENTS[index]) {
	    response = WS.sendRequest(findTestObject('Get Workflow Departments', [('BASE_URL') : GlobalVariable.BASE_URL, ('API_VERSION') : GlobalVariable.API_VERSION
	            , ('BUSINESS_TAG_ID') : GlobalVariable.BUSINESS_TAG_ID, ('WFID') : GlobalVariable.WFID, ('LOGIN_TOKEN') : GlobalVariable.LOGIN_TOKEN
	            , ('TAG') : DEPARTMENTS[index]]))
	
		WS.verifyResponseStatusCode(response, 200)
	
	    WS.verifyElementPropertyValue(response, 'error', 'false')
	
	    WS.containsString(response, 'data', false)
	
	    def jsonSlurper = new JsonSlurper()
	
	    def jsonResponse = jsonSlurper.parseText(response.getResponseText())
	
	    def elementsSize = jsonResponse.data.elements.size()
	
	    assert elementsSize > 0
		
		for (def index1 : (0..elementsSize)) {
			if(jsonResponse.data.elements[index1] && jsonResponse.data.elements[index1].cardtype == 'textcard' && jsonResponse.data.elements[index1].cardMeta.status == 'Active') {
				def wfMetaData = jsonSlurper.parseText(jsonResponse.data.elements[index1].wfMetaData)
				GlobalVariable.ACTIVE_STAGES.add(wfMetaData.tags.task)
			}
		}
	}
}
